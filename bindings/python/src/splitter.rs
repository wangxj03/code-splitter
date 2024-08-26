use pyo3::prelude::*;
use pyo3::types::PyBytes;
use tiktoken_rs::{cl100k_base, CoreBPE};
use tokenizers::Tokenizer;

use crate::chunk::Chunk;
use crate::language::Language;
use ::code_splitter::{CharCounter, Sizer, Splitter, WordCounter};

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
    #[pyo3(signature = (language, max_size))]
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
    #[pyo3(signature = (language, max_size))]
    fn new(language: Language, max_size: usize) -> PyResult<Self> {
        let splitter = GenericSplitter::new(language, max_size, WordCounter)?;
        Ok(WordSplitter(splitter))
    }

    fn split(&self, code: &Bound<'_, PyBytes>) -> PyResult<Vec<PyObject>> {
        self.0.split(code)
    }
}

#[pyclass]
pub struct TiktokenSplitter(GenericSplitter<CoreBPE>);

#[pymethods]
impl TiktokenSplitter {
    #[new]
    #[pyo3(signature = (language, max_size))]
    fn new(language: Language, max_size: usize) -> PyResult<Self> {
        let bpe = cl100k_base()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        let splitter = GenericSplitter::new(language, max_size, bpe)?;
        Ok(TiktokenSplitter(splitter))
    }

    fn split(&self, code: &Bound<'_, PyBytes>) -> PyResult<Vec<PyObject>> {
        self.0.split(code)
    }
}

#[pyclass]
pub struct HuggingfaceSplitter(GenericSplitter<Tokenizer>);

#[pymethods]
impl HuggingfaceSplitter {
    #[new]
    #[pyo3(signature = (language, max_size, pretrained_model_name_or_path))]
    fn new(
        language: Language,
        max_size: usize,
        pretrained_model_name_or_path: &str,
    ) -> PyResult<Self> {
        let tokenizer = Tokenizer::from_pretrained(pretrained_model_name_or_path, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        let splitter = GenericSplitter::new(language, max_size, tokenizer)?;
        Ok(HuggingfaceSplitter(splitter))
    }

    fn split(&self, code: &Bound<'_, PyBytes>) -> PyResult<Vec<PyObject>> {
        self.0.split(code)
    }
}
