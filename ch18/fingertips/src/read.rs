use std::{
    fs::{self, File},
    io::{self, BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::write::IndexFileWriter;

/// 파일을 처음부터 마지막까지 훑는다.
pub struct IndexFileReader {
    /// 실제의 인덱스 데이터
    main: BufReader<File>,
    /// 테이블
    contents: BufReader<File>,
    /// 테이블의 다음 엔트리, None이면 테이블의 마지막이다.
    next: Option<Entry>,
    filename: PathBuf,
}

/// 인덱스 파일의 테이블이다
pub struct Entry {
    /// 단어
    pub term: String,
    /// 단어의 전체 수
    pub df: u32,
    /// 인덱스 데이터의 시작 지점
    pub offset: u64,
    /// 인덱스 데이터의 길이
    pub nbytes: u64,
}

impl Drop for IndexFileReader {
    fn drop(&mut self) {
        let _ = self.delete();
    }
}

/// 파일 구조
/// [offset: 8B][hits(n):8 * n Byte](offset->|)[[hits_offset: 8B][hits_byte_len: 8B][hits_len: 4B][term_len: 4B][term: term_len B]...]
impl IndexFileReader {
    /// 인덱스 파일을 열어서 처음부터 마지막까지 읽는다.
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<IndexFileReader, io::Error> {
        let filename = filename.as_ref();
        let mut main_raw = File::open(filename)?;

        let content_offset = main_raw.read_u64::<LittleEndian>()?;
        println!(
            "opened {}, table of contents starts at {}",
            filename.display(),
            content_offset
        );
        let mut contents_raw = File::open(filename)?;
        contents_raw.seek(SeekFrom::Start(content_offset))?;

        let main = BufReader::new(main_raw);
        let mut contents = BufReader::new(contents_raw);

        let first = IndexFileReader::read_entry(&mut contents)?;

        Ok(IndexFileReader {
            main,
            contents,
            next: first,
            filename: filename.to_path_buf(),
        })
    }

    /// 파일 디스크립터 해제하고 파일을 삭제한다.
    pub fn delete(&mut self) -> Result<(), io::Error> {
        fs::remove_file(&self.filename)
    }

    /// 다음 Entry를 읽는다.
    pub fn read_entry(f: &mut BufReader<File>) -> Result<Option<Entry>, io::Error> {
        let offset = match f.read_u64::<LittleEndian>() {
            Ok(value) => value,
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    // 더 이상 읽을 것이 없다.
                    return Ok(None);
                } else {
                    return Err(e);
                }
            }
        };

        let nbytes = f.read_u64::<LittleEndian>()?;
        let df = f.read_u32::<LittleEndian>()?;
        let term_len = f.read_u32::<LittleEndian>()? as usize;
        let mut bytes = vec![0; term_len];
        f.read_exact(&mut bytes)?;
        let term = String::from_utf8(bytes)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "unicode fail"))?;

        Ok(Some(Entry {
            term,
            df,
            offset,
            nbytes,
        }))
    }

    /// 다음 엔트리의 참조를 빌려온다.
    pub fn peek(&self) -> Option<&Entry> {
        self.next.as_ref()
    }

    /// 다음 엔트리가 term과 일치할 때 true를 반환한다.
    pub fn is_at(&self, term: &str) -> bool {
        match self.next {
            Some(ref e) => e.term == term,
            None => false,
        }
    }

    pub fn move_entry_to(&mut self, out: &mut IndexFileWriter) -> Result<(), io::Error> {
        {
            let e = self.next.as_ref().expect("no entry to move");
            if e.nbytes > usize::MAX as u64 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "computer not big enough to hold index entry",
                ));
            }
            let mut buf = vec![0; e.nbytes as usize];
            self.main.read_exact(&mut buf)?;
            out.write_main(&buf)?;
        }

        self.next = Self::read_entry(&mut self.contents)?;

        Ok(())
    }
}
