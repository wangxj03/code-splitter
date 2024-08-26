use pyo3::prelude::*;
use pyo3::types::PyBytes;
use tree_sitter::Language;

use crate::chunk::Chunk;
use ::code_splitter::{CharCounter, Sizer, Splitter, WordCounter};

const DEFAULT_MAX_SIZE: usize = 512;

#[derive(Clone, Copy)]
enum SupportedLanguage {
    Golang,
    Markdown,
    Python,
    Rust,
}

impl SupportedLanguage {
    fn as_language(&self) -> Language {
        match self {
            SupportedLanguage::Golang => tree_sitter_go::language(),
            SupportedLanguage::Markdown => tree_sitter_md::language(),
            SupportedLanguage::Python => tree_sitter_python::language(),
            SupportedLanguage::Rust => tree_sitter_rust::language(),
        }
    }
}

struct GenericSplitter<T: Sizer> {
    splitter: Splitter<T>,
}

impl<T: Sizer> GenericSplitter<T> {
    fn new(language: &str, max_size: usize, sizer: T) -> PyResult<Self> {
        let supported_language = match language.to_lowercase().as_str() {
            "golang" => SupportedLanguage::Golang,
            "markdown" => SupportedLanguage::Markdown,
            "python" => SupportedLanguage::Python,
            "rust" => SupportedLanguage::Rust,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Unsupported language",
                ))
            }
        };

        let splitter = Splitter::new(supported_language.as_language(), sizer)
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
    fn new(language: &str, max_size: usize) -> PyResult<Self> {
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
    fn new(language: &str, max_size: usize) -> PyResult<Self> {
        let splitter = GenericSplitter::new(language, max_size, WordCounter)?;
        Ok(WordSplitter(splitter))
    }

    fn split(&self, code: &Bound<'_, PyBytes>) -> PyResult<Vec<PyObject>> {
        self.0.split(code)
    }
}
