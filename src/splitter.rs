#[cfg(feature = "tokenizers")]
mod huggingface;

#[cfg(feature = "tiktoken-rs")]
mod tiktoken;

mod words;

use crate::chunk::Chunk;
use crate::error::Result;
use std::str;
use tree_sitter::{Language, Node, Parser};

const DEFAULT_MAX_CHUNK_SIZE: usize = 512;

pub trait Sizer {
    fn size(&self, text: &str) -> Result<usize>;
}

/// A struct for splitting code into chunks.
pub struct Splitter<T: Sizer> {
    chunk_sizer: T,
    max_chunk_size: usize,
}

impl<T> Splitter<T>
where
    T: Sizer,
{
    pub fn with_max_chunk_size(mut self, max_chunk_size: usize) -> Self {
        self.max_chunk_size = max_chunk_size;
        self
    }

    /// Split the code into chunks with no larger than `max_chunk_size`.
    pub fn split(&self, code: &[u8], lang: &Language) -> Result<Vec<Chunk>> {
        if code.is_empty() {
            return Ok(vec![]);
        }

        let mut parser = Parser::new();
        parser.set_language(lang)?;
        let tree = parser.parse(code, None).ok_or("Failed to parse code")?;
        let root_node = tree.root_node();

        let chunks = self.split_node(&root_node, 0, code)?;

        Ok(chunks)
    }

    /// Split the code into chunks with no larger than `max_chunk_size`.
    fn split_node(&self, node: &Node, depth: usize, code: &[u8]) -> Result<Vec<Chunk>> {
        let text = node.utf8_text(code)?;
        let chunk_size = self.chunk_sizer.size(text)?;

        if chunk_size == 0 {
            return Ok(vec![]);
        }

        if chunk_size <= self.max_chunk_size {
            return Ok(vec![Chunk {
                subtree: format!("{}: {}", format_node(&node, depth), chunk_size),
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
                        if joined_size <= self.max_chunk_size {
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
        self.chunk_sizer.size(joined_text)
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
