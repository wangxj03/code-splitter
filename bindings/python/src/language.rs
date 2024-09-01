use pyo3::prelude::*;
use tree_sitter::Language as TreeSitterLanguage;

#[pyclass(eq)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum Language {
    Golang,
    Markdown,
    Python,
    Rust,
}

impl Language {
    pub fn as_tree_sitter_language(&self) -> TreeSitterLanguage {
        match self {
            Language::Golang => tree_sitter_go::language(),
            Language::Markdown => tree_sitter_md::language(),
            Language::Python => tree_sitter_python::language(),
            Language::Rust => tree_sitter_rust::language(),
        }
    }
}
