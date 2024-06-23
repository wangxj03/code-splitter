use tree_sitter::Language;

use crate::error::Result;
use crate::splitter::{Sizer, Splitter};

pub struct CharCounter;

impl Sizer for CharCounter {
    fn size(&self, text: &str) -> Result<usize> {
        Ok(text.chars().count())
    }
}

impl Splitter<CharCounter> {
    pub fn with_char_counter(language: Language) -> Result<Self> {
        Splitter::new(language, CharCounter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let counter = CharCounter;
        let text = "café";
        let size = counter.size(text).unwrap();
        assert_eq!(size, 4);
    }

    #[test]
    fn test_split_empty() {
        let code = b"";
        let lang = tree_sitter_md::language();
        let splitter = Splitter::with_char_counter(lang).unwrap();
        let chunks = splitter.split(code).unwrap();

        assert_eq!(chunks.len(), 0);
    }
}
