use std::{
    fs::{self, File},
    io::{self, BufWriter},
    path::{Path, PathBuf},
};

pub struct TmpDir {
    dir: PathBuf,
    n: usize,
}

impl TmpDir {
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        TmpDir {
            dir: dir.as_ref().to_owned(),
            n: 1,
        }
    }

    pub fn create(&mut self) -> Result<(PathBuf, BufWriter<File>), io::Error> {
        let mut r#try = 1;
        loop {
            let filename = self
                .dir
                .join(PathBuf::from(format!("tmp{:08x}.dat", self.n)));
            self.n += 1;
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&filename)
            {
                Ok(f) => return Ok((filename, BufWriter::new(f))),
                Err(exc) => {
                    if r#try < 999 && exc.kind() == io::ErrorKind::AlreadyExists {
                        // 계속 실행한다.
                    } else {
                        return Err(exc);
                    }
                }
            }
            r#try += 1;
        }
    }
}
