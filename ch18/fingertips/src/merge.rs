use std::{
    io,
    path::{Path, PathBuf},
};

use crate::tmp::TmpDir;

pub struct FileMerge {
    output_dir: PathBuf,
    tmp_dir: TmpDir,
    stacks: Vec<Vec<PathBuf>>,
}
impl FileMerge {
    pub fn new(output_dir: &Path) -> Self {
        todo!()
    }

    pub fn add_file(&mut self, mut file: PathBuf) -> Result<(), io::Error> {
        todo!()
    }

    pub fn finish(mut self) -> Result<(), io::Error> {
        todo!()
    }
}
