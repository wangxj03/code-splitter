use crate::error::Result;
use crate::sizer::Sizer;

/// A marker struct for counting words in code chunks.
///
/// ```
/// use code_splitter::{Splitter, WordCounter};
///
/// let lang = tree_sitter_md::language();
/// let splitter = Splitter::new(lang, WordCounter).unwrap();
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
