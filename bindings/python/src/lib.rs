mod chunk;
pub use chunk::Chunk;

mod splitter;
pub use splitter::{CharSplitter, WordSplitter};

use pyo3::prelude::*;

#[pymodule]
fn code_splitter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Chunk>()?;
    m.add_class::<CharSplitter>()?;
    m.add_class::<WordSplitter>()?;
    Ok(())
}
