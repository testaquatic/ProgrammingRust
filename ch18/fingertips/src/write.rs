use std::{io, path::PathBuf};

use crate::{index::InMemoryIndex, tmp::TmpDir};

pub struct IndexFileWriter {}

pub fn write_index_to_tmp_file(
    index: InMemoryIndex,
    tmp_dir: &mut TmpDir,
) -> Result<PathBuf, io::Error> {
    todo!()
}
