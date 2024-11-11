use std::{
    fs::{self, File},
    io::{self, BufWriter},
    mem,
    path::{Path, PathBuf},
};

use crate::{read::IndexFileReader, tmp::TmpDir, write::IndexFileWriter};

pub struct FileMerge {
    output_dir: PathBuf,
    tmp_dir: TmpDir,
    stacks: Vec<Vec<PathBuf>>,
}

/// 한번에 병합할 파일 수
const NSTREAMS: usize = 8;

const MERGED_FILENAME: &str = "index.dat";

impl FileMerge {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            tmp_dir: TmpDir::new(output_dir),
            stacks: Vec::new(),
        }
    }

    pub fn add_file(&mut self, mut file: PathBuf) -> Result<(), io::Error> {
        let mut level = 0;
        loop {
            if level == self.stacks.len() {
                self.stacks.push(Vec::new());
            }
            self.stacks[level].push(file);
            if self.stacks[level].len() < NSTREAMS {
                break Ok(());
            }
            let (filename, out) = self.tmp_dir.create()?;
            let mut to_merge = Vec::new();
            mem::swap(&mut self.stacks[level], &mut to_merge);
            merge_stream(to_merge, out)?;
            file = filename;
            level += 1;
        }
    }

    pub fn finish(mut self) -> Result<(), io::Error> {
        let mut tmp = Vec::with_capacity(NSTREAMS);
        self.stacks.into_iter().try_for_each(|statck| {
            statck.into_iter().rev().try_for_each(|file| {
                tmp.push(file);
                if tmp.len() == NSTREAMS {
                    return merge_reversed(&mut tmp, &mut self.tmp_dir);
                }
                Ok(())
            })
        })?;

        if tmp.len() > 1 {
            merge_reversed(&mut tmp, &mut self.tmp_dir)?;
        }
        assert!(tmp.len() <= 1);

        match tmp.pop() {
            Some(last_file) => fs::rename(last_file, self.output_dir.join(MERGED_FILENAME)),
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                "no documents were parsed or none contained any words",
            )),
        }
    }
}

fn merge_stream(files: Vec<PathBuf>, out: BufWriter<File>) -> Result<(), io::Error> {
    let mut streams = files
        .into_iter()
        .map(IndexFileReader::open)
        .collect::<Result<Vec<_>, _>>()?;

    let mut output = IndexFileWriter::new(out)?;

    let mut point = 0;
    let mut count = streams.iter().filter(|s| s.peek().is_some()).count();
    while count > 0 {
        let mut term = None;
        let mut nbytes = 0;
        let mut df = 0;
        streams.iter().filter_map(|s| s.peek()).for_each(|entry| {
            if term.is_none() || entry.term < *term.as_ref().unwrap() {
                term = Some(entry.term.clone());
                nbytes = entry.nbytes;
                df = entry.df;
            } else if entry.term == *term.as_ref().unwrap() {
                nbytes += entry.nbytes;
                df += entry.df;
            }
        });
        let term = term.expect("bug in algorithm!");

        streams
            .iter_mut()
            .filter(|s| s.is_at(&term))
            .try_for_each(|s| {
                s.move_entry_to(&mut *&mut output)?;
                if s.peek().is_none() {
                    count -= 1;
                }
                <Result<(), io::Error>>::Ok(())
            })?;

        output.write_content_entry(term, df, point, nbytes);
        point += nbytes;
    }

    assert!(streams.iter().all(|s| s.peek().is_none()));
    output.finish()
}

fn merge_reversed(filenames: &mut Vec<PathBuf>, tmp_dir: &mut TmpDir) -> io::Result<()> {
    filenames.reverse();
    let (merged_filename, out) = tmp_dir.create()?;
    let mut to_merge = Vec::with_capacity(NSTREAMS);
    mem::swap(filenames, &mut to_merge);
    merge_stream(to_merge, out)?;
    filenames.push(merged_filename);
    Ok(())
}
