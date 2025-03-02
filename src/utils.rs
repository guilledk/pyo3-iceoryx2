use pyo3::PyErr;

pub fn unwrap_or_pyerr<T>(val: Option<T>, err: PyErr) -> Result<T, PyErr> {
    if val.is_none() {
        Err(err)
    } else {
        Ok(val.unwrap())
    }
}

/*
 py_struct macro requires following imports:

    use pyo3::prelude::*;
    use pyo3::types::{PyAny, PyDict, Bound};
    use pyo3::exceptions::{PyKeyError, PyTypeError};
 */
#[macro_export]
macro_rules! py_struct {
    ($name:ident { $($field:ident : $type:ty),* $(,)? }) => {
        #[derive(Debug, Default)]
        pub struct $name {
            $(pub $field: $type),*
        }

        impl<'a> FromPyObject<'a> for $name {
            fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
                let py_dict = ob
                    .downcast::<PyDict>()
                    .map_err(|_| PyErr::new::<PyTypeError, _>(concat!("Expected a dict for ", stringify!($name))))?;

                Ok(Self {
                    $(
                        $field: match py_dict.get_item(stringify!($field))? {
                            Some(item) => item.extract()
                                .map_err(|_| PyErr::new::<PyTypeError, _>(concat!("Invalid type for ", stringify!($field))))?,
                            None => Default::default(), // Uses Default for missing Option<T> fields
                        },
                    )*
                })
            }
        }
    };
}
