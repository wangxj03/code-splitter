use pyo3::prelude::*;

#[pyclass]
pub struct Chunk {
    #[pyo3(get)]
    pub subtree: String,
    #[pyo3(get)]
    pub start: usize,
    #[pyo3(get)]
    pub end: usize,
    #[pyo3(get)]
    pub size: usize,
    #[pyo3(get)]
    pub text: String,
}
