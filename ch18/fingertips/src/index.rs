use std::collections::HashMap;

use byteorder::{LittleEndian, WriteBytesExt};

#[derive(Default)]
pub struct InMemoryIndex {
    // 문서의 단어수
    pub word_count: usize,
    pub map: HashMap<String, Vec<Hit>>,
}

pub type Hit = Vec<u8>;

impl InMemoryIndex {
    pub fn new() -> Self {
        Self::default()
    }

    // 문서의 인덱스를 생성한다.
    pub fn from_single_document(document_id: usize, text: String) -> InMemoryIndex {
        let document_id = document_id as u32;
        let text = text.to_lowercase();
        let tokens = tokenize(&text);
        let index =
            tokens
                .iter()
                .enumerate()
                .fold(InMemoryIndex::new(), |mut index, (i, token)| {
                    let hits = index.map.entry(token.to_string()).or_insert_with(|| {
                        // hit [document_id: 4B][i: B](n)
                        let mut hits: Vec<u8> = Vec::with_capacity(4 + 4);
                        hits.write_u32::<LittleEndian>(document_id).unwrap();
                        vec![hits]
                    });
                    hits[0].write_u32::<LittleEndian>(i as u32).unwrap();
                    index.word_count += 1;
                    index
                });

        if document_id % 100 == 0 {
            println!(
                "indexed document {}, {} bytes, {} words",
                document_id,
                text.len(),
                index.word_count
            );
        }

        index
    }

    pub fn merge(&mut self, other: InMemoryIndex) {
        other.map.into_iter().for_each(|(term, hits)| {
            self.map.entry(term).or_insert_with(Vec::new).extend(hits);
        });

        self.word_count += other.word_count;
    }

    // 인덱스의 크기가 크면 true를 반환한다.
    pub fn is_large(&self) -> bool {
        const REASONABLE_SIZE: usize = 100_000_000;
        self.word_count > REASONABLE_SIZE
    }

    // 인덱스가 비었을 때 true를 반환한다.
    pub fn is_empty(&self) -> bool {
        self.word_count == 0
    }
}

fn tokenize(text: &str) -> Vec<&str> {
    text.split(|ch: char| !ch.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .collect()
}
