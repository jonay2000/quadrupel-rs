use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use quadrupel_shared::message::*;

#[pyfunction]
pub fn parse_message_from_drone(message: &[u8]) -> PyResult<(String, usize)> {
    let (msg, len) =
        MessageToComputer::decode(message).map_err(|e| PyValueError::new_err(e.to_string()))?;

    Ok((
        serde_json::to_string(&msg).map_err(|e| PyValueError::new_err(e.to_string()))?,
        len,
    ))
}

#[pyfunction]
pub fn create_message_for_drone(json_str: &str) -> PyResult<Vec<u8>> {
    let str: MessageToDrone =
        serde_json::from_str(json_str).map_err(|e| PyValueError::new_err(e.to_string()))?;

    let v = str
        .encode_vec()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    Ok(v)
}

/// A Python module implemented in Rust.
#[pymodule]
fn quadrupel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_message_from_drone, m)?)?;
    m.add_function(wrap_pyfunction!(create_message_for_drone, m)?)?;
    Ok(())
}
