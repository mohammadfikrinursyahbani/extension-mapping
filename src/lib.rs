use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, FromPyObject)]
struct Mapping {
    #[pyo3(item)]
    id_responden: i64,
    #[pyo3(item)]
    kolom: String,
    #[pyo3(item)]
    label: i64,
}

#[pyfunction]
fn a_times_b(
    pydf: PyDataFrame,
    mapping: Vec<Mapping>,
) -> PyResult<PyDataFrame> {
    let df = pydf.as_ref();
    
    let mut conditions = Vec::new();
    for map in mapping.iter() {
        conditions.push(
            when(col("id_responden").eq(lit(map.id_responden)))
            .then(lit(map.label))
            .otherwise(col(&map.kolom))
            .alias(&map.kolom)
        );
    }

    let new_df = df.clone().lazy()
        .with_columns(conditions)
        .collect()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    Ok(PyDataFrame(new_df))
}

#[pymodule]
fn function_polars(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(a_times_b, m)?)?;
    Ok(())
}