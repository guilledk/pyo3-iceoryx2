pub mod utils;
pub mod pubsub;
pub mod event;
pub mod proxies;
pub mod globals;

use pyo3::prelude::*;

use crate::pubsub::{
    create_publisher,
    destroy_publisher,
    create_subscriber,
    destroy_subscriber,
    push, pop
};

use crate::event:: {
    create_notifier,
    destroy_notifier,
    create_listener,
    destroy_listener,
    notify,
    timed_wait_one,
    timed_wait_all
};


#[pymodule]
#[pyo3(name="_lowlevel")]
fn module_entrypoint(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_publisher, m)?)?;
    m.add_function(wrap_pyfunction!(create_subscriber, m)?)?;
    m.add_function(wrap_pyfunction!(destroy_publisher, m)?)?;
    m.add_function(wrap_pyfunction!(destroy_subscriber, m)?)?;
    m.add_function(wrap_pyfunction!(push, m)?)?;
    m.add_function(wrap_pyfunction!(pop, m)?)?;

    m.add_function(wrap_pyfunction!(create_notifier, m)?)?;
    m.add_function(wrap_pyfunction!(create_listener, m)?)?;
    m.add_function(wrap_pyfunction!(destroy_notifier, m)?)?;
    m.add_function(wrap_pyfunction!(destroy_listener, m)?)?;
    m.add_function(wrap_pyfunction!(notify, m)?)?;
    m.add_function(wrap_pyfunction!(timed_wait_one, m)?)?;
    m.add_function(wrap_pyfunction!(timed_wait_all, m)?)?;
    Ok(())
}
