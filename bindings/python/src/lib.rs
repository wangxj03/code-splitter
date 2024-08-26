mod chunk;
pub use chunk::Chunk;

mod language;
pub use language::Language;

mod splitter;
pub use splitter::{CharSplitter, HuggingfaceSplitter, TiktokenSplitter, WordSplitter};

use pyo3::prelude::*;

#[pymodule]
fn code_splitter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Chunk>()?;
    m.add_class::<Language>()?;
    m.add_class::<CharSplitter>()?;
    m.add_class::<HuggingfaceSplitter>()?;
    m.add_class::<TiktokenSplitter>()?;
    m.add_class::<WordSplitter>()?;
    Ok(())
}
