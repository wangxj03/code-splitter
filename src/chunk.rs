use std::fmt;
use tree_sitter::Range;

/// A chunk of code with a subtree and a range.
#[derive(Debug)]
pub struct Chunk {
    /// Subtree representation of the code chunk.
    pub subtree: String,
    /// Range of the code chunk.
    pub range: Range,
    /// Size of the code chunk.
    pub size: usize,
}

impl fmt::Display for Chunk {
    /// Display the chunk with its range and subtree.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{start}..{end}]: {size}\n{substree}",
            start = self.range.start_point.row,
            end = self.range.end_point.row,
            size = self.size,
            substree = self.subtree,
        )
    }
}

impl Chunk {
    pub fn utf8_lossy(&self, code: &[u8]) -> String {
        String::from_utf8_lossy(&code[self.range.start_byte as usize..self.range.end_byte])
            .to_string()
    }
}
