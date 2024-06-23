mod chars;
pub use chars::CharCounter;

#[cfg(feature = "tokenizers")]
mod huggingface;

#[cfg(feature = "tiktoken-rs")]
mod tiktoken;

mod words;
pub use words::WordCounter;

use crate::error::Result;

/// An interface for counting the size of a code chunk.
pub trait Sizer {
    fn size(&self, text: &str) -> Result<usize>;
}
