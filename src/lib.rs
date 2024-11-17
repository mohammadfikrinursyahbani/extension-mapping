use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, FromPyObject)]
struct Mapping {
    #[pyo3(item)]
    id_responden: i64,
    #[pyo3(item)]
    kolom: String,
    #[pyo3(item)]
    label: String,
}

#[pyfunction]
fn a_times_b(
    pydf: PyDataFrame,
    mapping: Vec<Mapping>,
) -> PyResult<PyDataFrame> {
    let df = pydf.as_ref();
    
    let mut column_conditions: HashMap<String, Vec<(i64, String)>> = HashMap::new();
    for map in mapping.iter() {
        column_conditions
            .entry(map.kolom.clone())
            .or_default()
            .push((map.id_responden, map.label.clone()));
    }
    
    let mut conditions = Vec::new();
    for (kolom, mappings) in column_conditions {
        let mut case_expr = col(&kolom);
        
        for (id_resp, label) in mappings.iter().rev() {
            case_expr = when(col("id_responden").eq(lit(*id_resp)))
                .then(lit(label.as_str()))  // Pass string slice directly
                .otherwise(case_expr);
        }
        
        conditions.push(case_expr.alias(&kolom));
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