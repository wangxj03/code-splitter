mod chunk;
mod error;
mod splitter;

pub use chunk::Chunk;
pub use error::{Error, Result};
pub use splitter::Splitter;

// #[cfg(feature = "tokenizers")]
// pub use splitter::Splitter::from_huggingface;

// #[cfg(feature = "tiktoken-rs")]
// pub use splitter::tiktoken::Splitter::from_tiktoken;
