use pyo3::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::OnceLock;
use tree_sitter::Language as TreeSitterLanguage;

#[pyclass(eq)]
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
#[non_exhaustive]
pub enum Language {
    Golang,
    Markdown,
    Python,
    Rust,
}

impl Eq for Language {}

static LANGUAGES: OnceLock<HashMap<Language, tree_sitter::Language>> = OnceLock::new();

impl Language {
    fn init_languages() -> HashMap<Language, tree_sitter::Language> {
        HashMap::from([
            (
                Language::Golang,
                tree_sitter::Language::new(tree_sitter_go::LANGUAGE),
            ),
            (
                Language::Markdown,
                tree_sitter::Language::new(tree_sitter_md::LANGUAGE),
            ),
            (
                Language::Python,
                tree_sitter::Language::new(tree_sitter_python::LANGUAGE),
            ),
            (
                Language::Rust,
                tree_sitter::Language::new(tree_sitter_rust::LANGUAGE),
            ),
        ])
    }

    pub fn as_tree_sitter_language(&self) -> TreeSitterLanguage {
        let languages = LANGUAGES.get_or_init(Self::init_languages);
        languages.get(self).unwrap().clone()
    }
}
