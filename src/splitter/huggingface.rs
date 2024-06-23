use tokenizers::Tokenizer;
use tree_sitter::Language;

use crate::error::Result;
use crate::splitter::{Sizer, Splitter};

impl Sizer for Tokenizer {
    fn size(&self, text: &str) -> Result<usize> {
        let encoding = self.encode(text, false)?;
        Ok(encoding.get_ids().len())
    }
}

impl Splitter<Tokenizer> {
    pub fn with_tokenizer(language: Language, tokenizer: Tokenizer) -> Result<Self> {
        Splitter::new(language, tokenizer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_size() {
        let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None).unwrap();
        let text = "I can feel the magic, can you?";
        let size = tokenizer.size(text).unwrap();
        assert_eq!(size, 9);
    }
}
