use tree_sitter::Language;

use crate::error::Result;
use crate::splitter::{Sizer, Splitter};

pub struct WordCounter;

impl Sizer for WordCounter {
    fn size(&self, text: &str) -> Result<usize> {
        Ok(text.split_whitespace().count())
    }
}

impl Splitter<WordCounter> {
    pub fn with_word_counter(language: Language) -> Result<Self> {
        Splitter::new(language, WordCounter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_size() {
        let counter = WordCounter;
        let text = "I can feel the magic, can you?";
        let size = counter.size(text).unwrap();
        assert_eq!(size, 7);
    }

    #[test]
    fn test_split_empty() {
        let code = b"";
        let lang = tree_sitter_md::language();
        let splitter = Splitter::with_word_counter(lang).unwrap();
        let chunks = splitter.split(code).unwrap();

        assert_eq!(chunks.len(), 0);
    }

    #[test]
    fn test_split_md() {
        let code = fs::read("testdata/sample.md").unwrap();
        let lang = tree_sitter_md::language();
        let max_words = 50;
        let splitter = Splitter::with_word_counter(lang)
            .unwrap()
            .with_max_size(max_words);
        let chunks = splitter.split(&code).unwrap();

        assert_eq!(chunks.len(), 5);
        chunks.into_iter().for_each(|chunk| {
            println!("{chunk}\n");
            assert!(chunk.size <= max_words);
        });
    }
}
