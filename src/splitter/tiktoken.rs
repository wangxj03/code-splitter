use tiktoken_rs::CoreBPE;

use crate::error::Result;
use crate::splitter::{Sizer, Splitter, DEFAULT_MAX_CHUNK_SIZE};

impl Sizer for CoreBPE {
    fn size(&self, text: &str) -> Result<usize> {
        let tokens = self.encode_with_special_tokens(text);
        Ok(tokens.len())
    }
}

impl Splitter<CoreBPE> {
    pub fn from_tiktoken(bpe: CoreBPE) -> Self {
        Splitter {
            chunk_sizer: bpe,
            max_chunk_size: DEFAULT_MAX_CHUNK_SIZE,
        }
    }
}

#[cfg(test)]
mod tests {
    use tiktoken_rs::cl100k_base;

    use super::*;

    #[test]
    fn test_core_bpe_size() {
        let bpe = cl100k_base().unwrap();
        let text = "I can feel the magic, can you?";
        let size = bpe.size(text).unwrap();
        assert_eq!(size, 9);
    }
}
