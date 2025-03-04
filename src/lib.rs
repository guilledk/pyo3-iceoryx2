pub mod utils;
pub mod pubsub;
pub mod event;
pub mod proxies;

use pyo3::prelude::*;

use crate::pubsub::{PyPublisher, PySubscriber};

use crate::event::{PyListener, PyNotifier};


#[pymodule]
#[pyo3(name="_lowlevel")]
fn module_entrypoint(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPublisher>()?;
    m.add_class::<PySubscriber>()?;
    m.add_class::<PyListener>()?;
    m.add_class::<PyNotifier>()?;
    Ok(())
}
