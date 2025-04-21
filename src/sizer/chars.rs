use crate::error::Result;
use crate::sizer::Sizer;

/// A marker struct for counting characters in code chunks.
///
/// ```
/// use tree_sitter::Language;
/// use code_splitter::{CharCounter, Splitter};
///
/// let splitter = Splitter::new(Language::new(tree_sitter_md::LANGUAGE), CharCounter).unwrap();
///
/// let code = b"hello, world!";
/// let chunks = splitter.split(code).unwrap();
/// ```
pub struct CharCounter;

impl Sizer for CharCounter {
    /// Count the number of characters in the given text.
    fn size(&self, text: &str) -> Result<usize> {
        Ok(text.chars().count())
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
}
