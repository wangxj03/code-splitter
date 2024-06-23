use tiktoken_rs::CoreBPE;

use crate::error::Result;
use crate::sizer::Sizer;

impl Sizer for CoreBPE {
    /// Count the number of tokens in the given text.
    fn size(&self, text: &str) -> Result<usize> {
        let tokens = self.encode_with_special_tokens(text);
        Ok(tokens.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tiktoken_rs::cl100k_base;

    #[test]
    fn test_core_bpe_size() {
        let bpe = cl100k_base().unwrap();
        let text = "I can feel the magic, can you?";
        let size = bpe.size(text).unwrap();
        assert_eq!(size, 9);
    }
}
