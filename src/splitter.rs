use crate::chunk::Chunk;
use crate::error::Result;
use crate::sizer::Sizer;

use std::str;
use tree_sitter::{Language, Node, Parser};

/// Default maximum size of a chunk.
const DEFAULT_MAX_SIZE: usize = 512;

/// A struct for splitting code into chunks.
pub struct Splitter<T: Sizer> {
    /// Language of the code.
    language: Language,
    /// Sizer for counting the size of code chunks.
    sizer: T,
    /// Maximum size of a code chunk.
    max_size: usize,
}

impl<T> Splitter<T>
where
    T: Sizer,
{
    /// Create a new `Splitter` that counts the size of code chunks with the given sizer.
    ///
    /// # Example: split by characters
    /// ```
    /// use code_splitter::{CharCounter, Splitter};
    ///
    /// let lang = tree_sitter_md::language();
    /// let splitter = Splitter::new(lang, CharCounter).unwrap();
    /// let chunks = splitter.split(b"hello, world!").unwrap();
    /// ```
    ///
    /// # Example: split by words
    /// ```
    /// use code_splitter::{Splitter, WordCounter};
    ///
    /// let lang = tree_sitter_md::language();
    /// let splitter = Splitter::new(lang, WordCounter).unwrap();
    /// let chunks = splitter.split(b"hello, world!").unwrap();
    /// ```
    ///
    /// # Example: split by tokens with huggingface tokenizer
    /// ```
    /// # #[cfg(feature = "tokenizers")]
    /// # {
    /// use code_splitter::Splitter;
    /// use tokenizers::Tokenizer;
    ///
    /// let lang = tree_sitter_md::language();
    /// let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None).unwrap();
    /// let splitter = Splitter::new(lang, tokenizer).unwrap();
    /// let chunks = splitter.split(b"hello, world!").unwrap();
    /// # }
    /// ```
    ///
    /// # Example: split by tokens with tiktoken core BPE
    /// ```
    /// # #[cfg(feature = "tiktoken-rs")]
    /// # {
    /// use code_splitter::Splitter;
    /// use tiktoken_rs::cl100k_base;
    ///
    /// let lang = tree_sitter_md::language();
    /// let bpe = cl100k_base().unwrap();
    /// let splitter = Splitter::new(lang, bpe).unwrap();
    /// let chunks = splitter.split(b"hello, world!").unwrap();
    /// # }
    /// ```
    pub fn new(language: Language, sizer: T) -> Result<Self> {
        // Ensure tree-sitter-<language> crate can be loaded
        Parser::new().set_language(&language)?;

        Ok(Self {
            language,
            sizer,
            max_size: DEFAULT_MAX_SIZE,
        })
    }

    /// Set the maximum size of a chunk. The default is 512.
    ///
    /// # Example: set the maximum size to 256
    /// ```
    /// use code_splitter::{CharCounter, Splitter};
    ///
    /// let lang = tree_sitter_md::language();
    /// let splitter = Splitter::new(lang, CharCounter)
    ///   .unwrap()
    ///   .with_max_size(256);
    /// let chunks = splitter.split(b"hello, world!").unwrap();
    /// ```
    pub fn with_max_size(mut self, max_size: usize) -> Self {
        self.max_size = max_size;
        self
    }

    /// Split the code into chunks with no larger than `max_size`.
    pub fn split(&self, code: &[u8]) -> Result<Vec<Chunk>> {
        if code.is_empty() {
            return Ok(vec![]);
        }

        let mut parser = Parser::new();
        parser
            .set_language(&self.language)
            .expect("Error loading tree-sitter language");
        let tree = parser.parse(code, None).ok_or("Error parsing code")?;
        let root_node = tree.root_node();

        let chunks = self.split_node(&root_node, 0, code)?;

        Ok(chunks)
    }

    fn split_node(&self, node: &Node, depth: usize, code: &[u8]) -> Result<Vec<Chunk>> {
        let text = node.utf8_text(code)?;
        let chunk_size = self.sizer.size(text)?;

        if chunk_size == 0 {
            return Ok(vec![]);
        }

        if chunk_size <= self.max_size {
            return Ok(vec![Chunk {
                subtree: format!("{}: {}", format_node(node, depth), chunk_size),
                range: node.range(),
                size: chunk_size,
            }]);
        }

        let chunks = node
            // Traverse the children in depth-first order
            .children(&mut node.walk())
            .map(|child| self.split_node(&child, depth + 1, code))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            // Join the tail and head of neighboring chunks if possible
            .try_fold(Vec::new(), |mut acc, mut next| -> Result<Vec<Chunk>> {
                if let Some(tail) = acc.pop() {
                    if let Some(head) = next.first_mut() {
                        let joined_size = self.joined_size(&tail, head, code)?;
                        if joined_size <= self.max_size {
                            // Concatenate the tail and head names
                            head.subtree = format!("{}\n{}", tail.subtree, head.subtree);
                            head.range.start_byte = tail.range.start_byte;
                            head.range.start_point = tail.range.start_point;
                            head.size = joined_size;
                        } else {
                            acc.push(tail);
                        }
                    } else {
                        // Push the tail back if next is empty
                        acc.push(tail);
                    }
                }
                acc.append(&mut next);
                Ok(acc)
            })?;

        Ok(chunks)
    }

    fn joined_size(&self, chunk: &Chunk, next: &Chunk, code: &[u8]) -> Result<usize> {
        let joined_bytes = &code[chunk.range.start_byte..next.range.end_byte];
        let joined_text = str::from_utf8(joined_bytes)?;
        self.sizer.size(joined_text)
    }
}

fn format_node(node: &Node, depth: usize) -> String {
    format!(
        "{indent}{branch} {kind:<32} [{start}..{end}]",
        indent = "│  ".repeat(depth.saturating_sub(1)),
        branch = if depth > 0 { "├─" } else { "" },
        kind = node.kind(),
        start = node.start_position().row,
        end = node.end_position().row
    )
}
