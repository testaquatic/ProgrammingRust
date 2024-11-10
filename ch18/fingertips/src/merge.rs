use std::{
    io, mem, path::{Path, PathBuf}
};

use crate::tmp::TmpDir;

pub struct FileMerge {
    output_dir: PathBuf,
    tmp_dir: TmpDir,
    stacks: Vec<Vec<PathBuf>>,
}

/// 한번에 병합할 파일 수
const NSTREAMS: usize = 8;

const MERGED_FILENAME: &'static str = "index.dat";

impl FileMerge {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            tmp_dir: TmpDir::new(output_dir.to_owned()),
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
            let (filename, out) = self.tmp_dir.create()?;
            let mut to_merge = Vec::new();
            mem::swap(&mut self.stacks[level], &mut to_merge);
        }
    }

    pub fn finish(mut self) -> Result<(), io::Error> {
        todo!()
    }
}
