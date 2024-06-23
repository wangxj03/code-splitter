use tokenizers::Tokenizer;

use crate::error::Result;
use crate::splitter::{Sizer, Splitter, DEFAULT_MAX_CHUNK_SIZE};

impl Sizer for Tokenizer {
    fn size(&self, text: &str) -> Result<usize> {
        let encoding = self.encode(text, false)?;
        Ok(encoding.get_ids().len())
    }
}

impl Splitter<Tokenizer> {
    pub fn from_huggingface(tokenizer: Tokenizer) -> Self {
        Splitter {
            chunk_sizer: tokenizer,
            max_chunk_size: DEFAULT_MAX_CHUNK_SIZE,
        }
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
