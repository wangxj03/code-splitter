use tokenizers::Tokenizer;

use crate::error::Result;
use crate::sizer::Sizer;

impl Sizer for Tokenizer {
    /// Count the number of tokens in the given text.
    fn size(&self, text: &str) -> Result<usize> {
        let encoding = self.encode(text, false)?;
        Ok(encoding.get_ids().len())
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
