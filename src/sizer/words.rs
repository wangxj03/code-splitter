use crate::error::Result;
use crate::sizer::Sizer;

/// A marker struct for counting words in code chunks.
///
/// ```
/// use tree_sitter::Language;
/// use code_splitter::{Splitter, WordCounter};
///
/// let splitter = Splitter::new(Language::new(tree_sitter_md::LANGUAGE), WordCounter).unwrap();
///
/// let code = b"hello, world!";
/// let chunks = splitter.split(code).unwrap();
/// ```
pub struct WordCounter;

impl Sizer for WordCounter {
    /// Count the number of words in the given text.
    fn size(&self, text: &str) -> Result<usize> {
        Ok(text.split_whitespace().count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let counter = WordCounter;
        let text = "hello, world!";
        let size = counter.size(text).unwrap();
        assert_eq!(size, 2);
    }
}
