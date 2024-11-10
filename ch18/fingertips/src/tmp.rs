use std::path::{Path, PathBuf};

pub struct TmpDir {
    dir: PathBuf,
    n: usize,
}

impl TmpDir {
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        todo!()
    }
}
