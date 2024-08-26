use pyo3::prelude::*;
use pyo3::types::PyBytes;
use tree_sitter::Language as TreeSitterLanguage;

use crate::chunk::Chunk;
use ::code_splitter::{CharCounter, Sizer, Splitter, WordCounter};

const DEFAULT_MAX_SIZE: usize = 512;

#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Language {
    Golang,
    Markdown,
    Python,
    Rust,
}

impl Language {
    fn as_tree_sitter_language(&self) -> TreeSitterLanguage {
        match self {
            Language::Golang => tree_sitter_go::language(),
            Language::Markdown => tree_sitter_md::language(),
            Language::Python => tree_sitter_python::language(),
            Language::Rust => tree_sitter_rust::language(),
        }
    }
}

struct GenericSplitter<T: Sizer> {
    splitter: Splitter<T>,
}

impl<T: Sizer> GenericSplitter<T> {
    fn new(language: Language, max_size: usize, sizer: T) -> PyResult<Self> {
        let splitter = Splitter::new(language.as_tree_sitter_language(), sizer)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?
            .with_max_size(max_size);

        Ok(GenericSplitter { splitter })
    }

    fn split(&self, code: &Bound<'_, PyBytes>) -> PyResult<Vec<PyObject>> {
        let code_bytes = code.as_bytes();
        let chunks = self
            .splitter
            .split(code_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Python::with_gil(|py| {
            chunks
                .into_iter()
                .map(|chunk| {
                    let chunk = Chunk {
                        subtree: chunk.subtree.to_string(),
                        start: chunk.range.start_point.row,
                        end: chunk.range.end_point.row,
                        size: chunk.size,
                        text: chunk.utf8_lossy(code_bytes),
                    };
                    Ok(chunk.into_py(py))
                })
                .collect::<Result<Vec<PyObject>, PyErr>>()
        })
    }
}

#[pyclass]
pub struct CharSplitter(GenericSplitter<CharCounter>);

#[pymethods]
impl CharSplitter {
    #[new]
    #[pyo3(signature = (language, max_size = DEFAULT_MAX_SIZE))]
    fn new(language: Language, max_size: usize) -> PyResult<Self> {
        let splitter = GenericSplitter::new(language, max_size, CharCounter)?;
        Ok(CharSplitter(splitter))
    }

    fn split(&self, code: &Bound<'_, PyBytes>) -> PyResult<Vec<PyObject>> {
        self.0.split(code)
    }
}

#[pyclass]
pub struct WordSplitter(GenericSplitter<WordCounter>);

#[pymethods]
impl WordSplitter {
    #[new]
    #[pyo3(signature = (language, max_size = DEFAULT_MAX_SIZE))]
    fn new(language: Language, max_size: usize) -> PyResult<Self> {
        let splitter = GenericSplitter::new(language, max_size, WordCounter)?;
        Ok(WordSplitter(splitter))
    }

    fn split(&self, code: &Bound<'_, PyBytes>) -> PyResult<Vec<PyObject>> {
        self.0.split(code)
    }
}