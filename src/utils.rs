use pyo3::PyErr;

pub fn unwrap_or_pyerr<T>(val: Option<T>, err: PyErr) -> Result<T, PyErr> {
    if val.is_none() {
        Err(err)
    } else {
        Ok(val.unwrap())
    }
}