use libloading::{Library, Symbol};
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
    #[pyo3(signature = (language, max_size = None))]
    fn new(language: &str, max_size: Option<usize>) -> PyResult<Self> {
        let lang = load_language(language)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;

        let splitter = Splitter::new(lang, CharCounter)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?
            .with_max_size(max_size.unwrap_or(DEFAULT_MAX_SIZE));

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

fn load_language(language: &str) -> Result<Language, String> {
    let lib_name = format!("tree-sitter-{}", language);
    let lib =
        unsafe { Library::new(&lib_name).map_err(|e| format!("Failed to load library: {}", e))? };

    unsafe {
        let language_fn: Symbol<unsafe extern "C" fn() -> Language> = lib
            .get(b"tree_sitter_language")
            .map_err(|e| format!("Failed to find tree_sitter_language function: {}", e))?;

        Ok(language_fn())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn code_splitter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Chunk>()?;
    m.add_class::<CharSplitter>()?;
    Ok(())
}
