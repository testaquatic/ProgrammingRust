use clap::{Arg, ArgAction, Command};

/// 인수 목록
#[derive(Debug)]
pub struct Args {
    single_threaded: bool,
    filename: Vec<String>,
}

impl Args {
    pub fn is_single_threaded(&self) -> bool {
        self.single_threaded
    }

    pub fn filename(&self) -> &[String] {
        &self.filename
    }
}

/// 커맨드라인에서 Args를 생성한다.
pub fn parse_args() -> Args {
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
