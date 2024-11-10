use std::collections::HashMap;

pub struct InMemoryIndex {
    // 문서의 단어수
    pub word_count: usize,
    pub map: HashMap<String, Vec<Hit>>,
}

pub type Hit = Vec<u8>;

impl InMemoryIndex {
    pub fn new() -> Self {
        todo!()
    }
    pub fn from_single_document(document_id: usize, text: String) -> InMemoryIndex {
        todo!()
    }

    pub fn merge(&mut self, other: InMemoryIndex) {
        todo!()
    }
    pub fn is_large(&self) -> bool {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
}
