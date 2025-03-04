use std::time::Duration;
use iceoryx2::node::{Node, NodeBuilder};
use iceoryx2::prelude::ipc::Service;
use iceoryx2::prelude::{AttributeVerifier, EventId, ServiceName};
use iceoryx2::service::port_factory::event::PortFactory;
use pyo3::{pyfunction, PyResult};
use pyo3::exceptions::{PyKeyError, PyOSError, PyValueError};
use crate::globals::{SafeNotifier, SafeListener, NOTIFIERS, LISTENERS};
use crate::proxies::PyServiceConfig;
use crate::utils::unwrap_or_pyerr;


fn open_or_create_service(name: &str, node: &Node<Service>, spec: &AttributeVerifier) -> PyResult<PortFactory<Service>> {
    Ok(node
        .service_builder(
            &ServiceName::new(name).map_err(|e| PyOSError::new_err(
                format!("Could not instantiate service name {}: {}", name, e)))?)
        .event()
        .open_or_create_with_attributes(spec)
        .map_err(|e| PyOSError::new_err(
            format!("Could not open event service {}: {}", name, e)))?)
}


#[pyfunction]
#[pyo3(signature = (name, service_config=None))]
pub fn create_notifier(
    name: &str,
    service_config: Option<PyServiceConfig>,
) -> PyResult<()> {
    let service_config = service_config.unwrap_or_default();
    let mut notifiers = NOTIFIERS.lock()
        .map_err(|e|
            PyOSError::new_err(
                format!("Failed to lock NOTIFIERS mutex: {}", e)))?;

    if notifiers.contains_key(name) {
        return Err(
            PyValueError::new_err(format!("Notifier with name '{}' already exists", name)));
    }

    let node = NodeBuilder::new().create::<Service>()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to create node: {}", e)))?;

    let service = open_or_create_service(name, &node, &service_config.into())?;

    let notifier = service
        .notifier_builder()
        .create()
        .map_err(|e| PyOSError::new_err(
            format!("Could not create notification service {}: {}", name, e)))?;

    notifiers.insert(name.to_string(), SafeNotifier(notifier));

    Ok(())
}

#[pyfunction]
pub fn destroy_notifier(name: &str) -> PyResult<bool> {
    let mut notifiers = NOTIFIERS.lock()
        .map_err(|e|
            PyOSError::new_err(
                format!("Failed to lock NOTIFIERS mutex: {}", e)))?;

    match notifiers.remove(name) {
        None => Ok(false),
        Some(_) => Ok(true)
    }
}

#[pyfunction]
pub fn notify(
    name: &str,
    event_id: usize
) -> PyResult<()> {
    let notifiers = NOTIFIERS.lock()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to lock NOTIFIERS mutex: {}", e)))?;

    let notifier = unwrap_or_pyerr(
        notifiers.get(&name.to_string()),
        PyKeyError::new_err(format!("Publisher not found {}", name)),
    )?;

    notifier.0.notify_with_custom_event_id(EventId::new(event_id))
        .map_err(|e| PyOSError::new_err(
            format!("Failed to notfiy event: {}", e)))?;

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (name, service_config=None))]
pub fn create_listener(
    name: &str,
    service_config: Option<PyServiceConfig>,
) -> PyResult<()> {
    let service_config = service_config.unwrap_or_default();
    let mut listeners = LISTENERS.lock()
        .map_err(|e|
            PyOSError::new_err(
                format!("Failed to lock LISTENERS mutex: {}", e)))?;

    if listeners.contains_key(name) {
        return Err(
            PyValueError::new_err(format!("Listener with name '{}' already exists", name)));
    }

    let node = NodeBuilder::new().create::<Service>()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to create node: {}", e)))?;

    let service = open_or_create_service(name, &node, &service_config.into())?;

    let listener = service
        .listener_builder()
        .create()
        .map_err(|e| PyOSError::new_err(
            format!("Could not create notification service {}: {}", name, e)))?;

    listeners.insert(name.to_string(), SafeListener(listener));

    Ok(())
}

#[pyfunction]
pub fn destroy_listener(name: &str) -> PyResult<bool> {
    let mut listeners = LISTENERS.lock()
        .map_err(|e|
            PyOSError::new_err(
                format!("Failed to lock LISTENERS mutex: {}", e)))?;

    match listeners.remove(name) {
        None => Ok(false),
        Some(_) => Ok(true)
    }
}

#[pyfunction]
pub fn timed_wait_one(
    name: &str,
    timeout: u64,
) -> PyResult<Option<usize>> {
    let listeners = LISTENERS.lock()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to lock LISTENERS mutex: {}", e)))?;

    let listener = unwrap_or_pyerr(
        listeners.get(&name.to_string()),
        PyKeyError::new_err(format!("Listener not found {}", name)),
    )?;

    let event = listener.0.timed_wait_one(Duration::from_millis(timeout))
        .map_err(|e| PyOSError::new_err(
            format!("Timed wait all event failed: {}", e)))?
        .map(|event| event.as_value());

    Ok(event)
}

#[pyfunction]
pub fn timed_wait_all(
    name: &str,
    timeout: u64,
) -> PyResult<Vec<usize>> {
    let listeners = LISTENERS.lock()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to lock LISTENERS mutex: {}", e)))?;

    let listener = unwrap_or_pyerr(
        listeners.get(&name.to_string()),
        PyKeyError::new_err(format!("Listener not found {}", name)),
    )?;

    let mut events = Vec::new();
    let collect_event = |e: EventId| events.push(e.as_value());

    listener.0.timed_wait_all(collect_event, Duration::from_millis(timeout))
        .map_err(|e| PyOSError::new_err(
            format!("Timed wait all event failed: {}", e)))?;

    Ok(events)
}