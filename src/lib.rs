use ndarray::{Array1, Array2, arr2};
use pyo3::prelude::*;
use pyo3::types::{PyList};
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyValueError;
use pyo3::{pymodule, types::PyModule, PyResult, Python};

// src/lib.rs
mod rmsd;
mod utils;


#[pymodule]
fn rust_calc(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(add, m)?)?;
}


#[pyfunction]
fn calculate_rmsd_py(
    structure1: Vec<f64>,
    structure2: Vec<f64>,
    fit_sel1: Vec<usize>,
    fit_sel2: Vec<usize>,
    rmsd_sel1: Vec<usize>,
    rmsd_sel2: Vec<usize>,
) -> f64 {
    let fitted_structure1 = fit_structure(&structure1, &structure2, &fit_sel1, &fit_sel2);
    calculate_rmsd(&fitted_structure1, &structure2, &rmsd_sel1, &rmsd_sel2)
}

#[pymodule]
fn my_rust_lib(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_rmsd_py, m)?)?;
    Ok(())
}
