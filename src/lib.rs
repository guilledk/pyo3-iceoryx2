pub mod utils;
pub mod pubsub;
pub mod event;
pub mod proxies;

use pyo3::prelude::*;

use crate::pubsub::{
    create_publisher,
    create_subscriber,
    push, pop
};

use crate::event:: {
    create_notifier,
    create_listener,
    notify, timed_wait_all
};


#[pymodule]
fn pyo3_iceoryx2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_publisher, m)?)?;
    m.add_function(wrap_pyfunction!(create_subscriber, m)?)?;
    m.add_function(wrap_pyfunction!(push, m)?)?;
    m.add_function(wrap_pyfunction!(pop, m)?)?;

    m.add_function(wrap_pyfunction!(create_notifier, m)?)?;
    m.add_function(wrap_pyfunction!(create_listener, m)?)?;
    m.add_function(wrap_pyfunction!(notify, m)?)?;
    m.add_function(wrap_pyfunction!(timed_wait_all, m)?)?;
    Ok(())
}
