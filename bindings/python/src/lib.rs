use pyo3::prelude::*;
use pyo3::types::PyBytes;
use tree_sitter::Language;

use ::code_splitter::{CharCounter, Splitter};

const DEFAULT_MAX_SIZE: usize = 512;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

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

#[pyclass]
struct Chunk {
    #[pyo3(get)]
    subtree: String,
    #[pyo3(get)]
    start: usize,
    #[pyo3(get)]
    end: usize,
    #[pyo3(get)]
    size: usize,
    #[pyo3(get)]
    text: String,
}

#[pyclass]
struct CharSplitter {
    splitter: Splitter<CharCounter>,
}

#[pymethods]
impl CharSplitter {
    #[new]
    #[pyo3(signature = (language, max_size = DEFAULT_MAX_SIZE))]
    fn new(language: &str, max_size: usize) -> PyResult<Self> {
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

        let splitter = Splitter::new(supported_language.as_language(), CharCounter)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?
            .with_max_size(max_size);

        Ok(CharSplitter { splitter })
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

#[pymodule]
fn code_splitter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Chunk>()?;
    m.add_class::<CharSplitter>()?;
    Ok(())
}
