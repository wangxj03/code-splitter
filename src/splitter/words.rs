use crate::error::Result;
use crate::splitter::{Sizer, Splitter, DEFAULT_MAX_CHUNK_SIZE};

pub struct Words;

impl Sizer for Words {
    fn size(&self, text: &str) -> Result<usize> {
        Ok(text.split_whitespace().count())
    }
}

impl Splitter<Words> {
    pub fn words() -> Self {
        Splitter {
            chunk_sizer: Words,
            max_chunk_size: DEFAULT_MAX_CHUNK_SIZE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_size() {
        let words = Words;
        let text = "I can feel the magic, can you?";
        let size = words.size(text).unwrap();
        assert_eq!(size, 7);
    }

    #[test]
    fn test_split_empty() {
        let code = b"";
        let lang = tree_sitter_md::language();
        let splitter = Splitter::words();
        let chunks = splitter.split(code, &lang).unwrap();

        assert_eq!(chunks.len(), 0);
    }

    #[test]
    fn test_split_md() {
        let code = fs::read("testdata/sample.md").unwrap();
        let lang = tree_sitter_md::language();
        let max_chunk_size = 50;
        let splitter = Splitter::words().with_max_chunk_size(max_chunk_size);
        let chunks = splitter.split(&code, &lang).unwrap();

        assert_eq!(chunks.len(), 5);
        chunks.into_iter().for_each(|chunk| {
            println!("{chunk}\n");
            assert!(chunk.size <= max_chunk_size);
        });
    }
}
