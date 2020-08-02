use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;

/// Gets a dict
#[pyfunction]
fn sum_as_string(a: &PyDict) {
    for item in a {
        println!("{} {}", item.0, item.1);
    };
}

/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;

    Ok(())
}