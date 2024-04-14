use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn say_hello() {
    println!("Saying hello from Rust!")
}

/// A Python module implemented in Rust.
#[pymodule]
fn mat_flitton_fib_py_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello));
    Ok(())
}
