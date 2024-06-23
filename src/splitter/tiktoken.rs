use tiktoken_rs::CoreBPE;
use tree_sitter::Language;

use crate::error::Result;
use crate::splitter::{Sizer, Splitter};

impl Sizer for CoreBPE {
    /// Count the number of tokens in the given text.
    fn size(&self, text: &str) -> Result<usize> {
        let tokens = self.encode_with_special_tokens(text);
        Ok(tokens.len())
    }
}

impl Splitter<CoreBPE> {
    /// Create a new `Splitter` that counts tokens with the given core BPE in code chunks.
    ///
    /// ```
    /// use code_splitter::Splitter;
    /// use tiktoken_rs::cl100k_base;
    ///
    /// let lang = tree_sitter_md::language();
    /// let bpe = cl100k_base().unwrap();
    /// let splitter = Splitter::with_core_bpe(lang, bpe).unwrap();
    ///
    /// let code = b"hello, world!";
    /// let chunks = splitter.split(code).unwrap();
    /// ```
    pub fn with_core_bpe(language: Language, bpe: CoreBPE) -> Result<Self> {
        Splitter::new(language, bpe)
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
