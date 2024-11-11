use std::{
    fs::File,
    io::{self, BufWriter, Seek, Write},
    path::PathBuf,
};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::{index::InMemoryIndex, tmp::TmpDir};

// 인덱스를 파일에 저장하기 위한 Writer
pub struct IndexFileWriter {
    offset: u64,
    writer: BufWriter<File>,
    content_buf: Vec<u8>,
}

/// 파일 구조
/// [offset: 8B][hits(n):8 * n Byte](offset->|)[[hits_offset: 8B][hits_byte_len: 8B][hits_len: 4B][term_len: 4B][term: term_len B]...]
impl IndexFileWriter {
    pub fn new(mut f: BufWriter<File>) -> Result<IndexFileWriter, io::Error> {
        const HEADER_SIZE: u64 = 8;
        f.write_u64::<LittleEndian>(0)?;
        Ok(IndexFileWriter {
            offset: HEADER_SIZE,
            writer: f,
            content_buf: Vec::new(),
        })
    }

    /// writer에 buf의 내용을 쓴다.
    pub fn write_main(&mut self, buf: &[u8]) -> Result<(), io::Error> {
        self.writer.write_all(buf)?;
        self.offset += buf.len() as u64;

        Ok(())
    }

    // content_buf에 쓴다.
    pub fn write_content_entry(&mut self, term: String, df: u32, offset: u64, nbytes: u64) {
        self.content_buf.write_u64::<LittleEndian>(offset).unwrap();
        self.content_buf.write_u64::<LittleEndian>(nbytes).unwrap();
        self.content_buf.write_u32::<LittleEndian>(df).unwrap();
        let bytes = term.bytes();
        self.content_buf
            .write_u32::<LittleEndian>(bytes.len() as u32)
            .unwrap();
        self.content_buf.extend(bytes);
    }

    pub fn finish(&mut self) -> Result<(), io::Error> {
        let content_start = self.offset;
        self.writer.write_all(&self.content_buf)?;
        println!(
            "{} bytes main, {} bytes total",
            content_start,
            content_start + self.content_buf.len() as u64
        );
        self.writer.seek(io::SeekFrom::Start(0))?;
        self.writer.write_u64::<LittleEndian>(content_start)?;

        Ok(())
    }
}

pub fn write_index_to_tmp_file(
    index: InMemoryIndex,
    tmp_dir: &mut TmpDir,
) -> Result<PathBuf, io::Error> {
    let (filename, f) = tmp_dir.create()?;
    let mut writer = IndexFileWriter::new(f)?;

    let mut index_as_vec = index.map.into_iter().collect::<Vec<_>>();
    index_as_vec.sort_by(|(a, _), (b, _)| a.cmp(b));

    index_as_vec.into_iter().try_for_each(|(term, hits)| {
        let df = hits.len() as u32;
        let start = writer.offset;
        hits.into_iter()
            .try_for_each(|buffer| writer.write_main(&buffer))?;
        let stop = writer.offset;
        writer.write_content_entry(term, df, start, stop - start);

        <Result<(), io::Error>>::Ok(())
    })?;

    writer.finish()?;
    println!("wrote file {:?}", filename);

    Ok(filename)
}
