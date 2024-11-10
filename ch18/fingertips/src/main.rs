mod index;
mod merge;
mod tmp;
mod write;

use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use clap::{Arg, ArgAction, Command};
use index::InMemoryIndex;
use merge::FileMerge;
use tmp::TmpDir;
use write::write_index_to_tmp_file;

/// 인수 목록
#[derive(Debug)]
struct Args {
    single_threaded: bool,
    filename: Vec<String>,
}

/// 커맨드라인에서 Args를 생성한다.
fn parse_args() -> Args {
    let matches = Command::new("fingertips")
        .about("문서의 역색인을 만든다.")
        .version("0.1.0")
        .arg(
            Arg::new("single_threaded")
                .help("싱글스레드로 작업을 실행한다.")
                .action(ArgAction::SetTrue)
                .short('1')
                .long("single-threaded"),
        )
        .arg(
            Arg::new("filenames")
                .required(true)
                .help(
                    "인덱스를 생성할 파일이나 디렉터리 이름. \
                디렉터리를 지정한 경우에는 디렉터리 아래의 모든 .txt 파일을 인덱스 한다.",
                )
                .num_args(1..),
        )
        .get_matches();

    let args = Args {
        single_threaded: *matches.get_one("single_threaded").unwrap_or(&false),
        filename: matches.get_many("filenames").unwrap().cloned().collect(),
    };

    args
}

/// 순회할 파일 목록을 작성한다.
/// `io::Result<()>` 보다 `Result<(), io::Error>` 이 가독성이 나은 것 같아서 대신 사용한다.
/// for 보다 try_fold나 try_for_each이 보기 좋은 것 같아서 대신 사용한다.
/// 흐름을 따라서 쭉 읽으면 코드가 바로 파악된다.
fn expand_filename_arguments(args_filenames: Vec<String>) -> Result<Vec<PathBuf>, io::Error> {
    args_filenames
        .into_iter()
        .map(|args_filename| PathBuf::from(args_filename))
        .try_fold(vec![], |mut filenames, path| {
            if path.metadata()?.is_dir() {
                path.read_dir()?.into_iter().try_for_each(|entry| {
                    let entry = entry?;
                    if entry.file_type()?.is_file() {
                        filenames.push(entry.path());
                    }
                    <Result<(), io::Error>>::Ok(())
                })?;
            } else {
                filenames.push(path);
            }

            Ok(filenames)
        })
}

/// 싱글스레드에서 역인덱스를 생성한다.
fn run_single_threaded(documents: Vec<PathBuf>, output_dir: PathBuf) -> Result<(), io::Error> {
    let mut accumulated_index = InMemoryIndex::new();
    let mut merge = FileMerge::new(&output_dir);
    let mut tmp_dir = TmpDir::new(&output_dir);

    documents
        .into_iter()
        .map(|filename| {
            let mut f = File::open(filename)?;
            let mut text = String::new();
            f.read_to_string(&mut text)?;
            <Result<String, io::Error>>::Ok(text)
        })
        .enumerate()
        .try_for_each(|(doc_id, text_result)| {
            let index = InMemoryIndex::from_single_document(doc_id, text_result?);
            accumulated_index.merge(index);
            if accumulated_index.is_large() {
                let file = write_index_to_tmp_file(
                    std::mem::replace(&mut accumulated_index, InMemoryIndex::new()),
                    &mut tmp_dir,
                )?;
                merge.add_file(file)?;
            }
            <Result<(), io::Error>>::Ok(())
        })?;

    if !accumulated_index.is_empty() {
        let file = write_index_to_tmp_file(accumulated_index, &mut tmp_dir)?;
        merge.add_file(file)?;
    }
    merge.finish()
}

fn run_pipeline(documents: Vec<PathBuf>, output_dir: PathBuf) -> io::Result<()> {
    todo!()
}

fn run(args: Args) -> Result<(), io::Error> {
    let output_dir = PathBuf::from(".");
    let (filenames, single_threaded) = (args.filename, args.single_threaded);
    let documents = expand_filename_arguments(filenames)?;
    if single_threaded {
        run_single_threaded(documents, output_dir)
    } else {
        run_pipeline(documents, output_dir)
    }
}

fn main() {
    let args = parse_args();
    run(args).unwrap_or_else(|e| println!("error: {}", e));
}
