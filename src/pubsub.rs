use iceoryx2::prelude::ipc::Service;
use iceoryx2::prelude::{AttributeVerifier, Node, NodeBuilder, ServiceName};
use iceoryx2::service::port_factory::publish_subscribe::PortFactory;
use pyo3::exceptions::{PyKeyError, PyOSError, PyValueError};
use pyo3::prelude::*;
use crate::globals::{SafePublisher, SafeSubscriber, PUBLISHERS, SUBSCRIBERS};
use crate::proxies::{PyPublisherConfig, PyServiceConfig, PySubscriberConfig};
use crate::utils::unwrap_or_pyerr;


fn open_or_create_service(name: &str, node: &Node<Service>, spec: &AttributeVerifier) -> PyResult<PortFactory<Service, [u8], ()>> {
    Ok(node
        .service_builder(
            &ServiceName::new(name).map_err(|e| PyOSError::new_err(
                format!("Could not instantiate service name {}: {}", name, e)))?)
        .publish_subscribe::<[u8]>()
        .open_or_create_with_attributes(spec)
        .map_err(|e| PyOSError::new_err(
            format!("Failed to open service: {}", e)))?)
}

#[pyfunction]
#[pyo3(signature = (name, service_config=None, publisher_config=None))]
pub fn create_publisher(
    name: &str,
    service_config: Option<PyServiceConfig>,
    publisher_config: Option<PyPublisherConfig>,
) -> PyResult<()> {
    let service_config = service_config.unwrap_or_default();
    let publisher_config = publisher_config.unwrap_or_default();

    let mut publishers = PUBLISHERS.lock()
        .map_err(|e|
            PyOSError::new_err(
                format!("Failed to lock PUBLISHERS mutex: {}", e)))?;

    if publishers.contains_key(name) {
        return Err(
            PyValueError::new_err(format!("Publisher with name '{}' already exists", name)));
    }

    let node = NodeBuilder::new().create::<Service>()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to create node: {}", e)))?;

    let service = open_or_create_service(name, &node, &service_config.into())?;

    let publisher = service
        .publisher_builder()
        .initial_max_slice_len(publisher_config.initial_max_slice_len())
        .allocation_strategy(publisher_config.allocation_strategy()?)
        .unable_to_deliver_strategy(publisher_config.unable_to_deliver_strategy()?)
        .max_loaned_samples(publisher_config.max_loaned_samples())
        .create()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to create publisher: {}", e)))?;

    publishers.insert(name.to_string(), SafePublisher(publisher));

    Ok(())
}

#[pyfunction]
pub fn destroy_publisher(name: &str) -> PyResult<bool> {
    let mut publishers = PUBLISHERS.lock()
        .map_err(|e|
            PyOSError::new_err(
                format!("Failed to lock PUBLISHERS mutex: {}", e)))?;

    match publishers.remove(name) {
        None => Ok(false),
        Some(_) => Ok(true)
    }
}

#[pyfunction]
pub fn push(
    name: &str,
    data: &[u8],
) -> PyResult<usize> {
    let publishers = PUBLISHERS.lock()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to lock PUBLISHERS mutex: {}", e)))?;

    let publisher = unwrap_or_pyerr(
        publishers.get(&name.to_string()),
        PyKeyError::new_err(format!("Publisher not found {}", name)),
    )?;

    let slice = publisher.0.loan_slice_uninit(data.len())
        .map_err(|e| PyOSError::new_err(
            format!("Failed to loan slice: {}", e)))?;

    let slice = slice.write_from_slice(data);
    Ok(slice.send()
        .map_err(
            |e| PyKeyError::new_err(
                format!("Failed to send data: {}", e)))?)
}

#[pyfunction]
#[pyo3(signature = (name, service_config=None, subscriber_config=None))]
pub fn create_subscriber(
    name: &str,
    service_config: Option<PyServiceConfig>,
    subscriber_config: Option<PySubscriberConfig>,
) -> PyResult<()> {
    let service_config = service_config.unwrap_or_default();
    let subscriber_config = subscriber_config.unwrap_or_default();
    let mut subscribers = SUBSCRIBERS.lock()
        .map_err(|e|
            PyOSError::new_err(
                format!("Failed to lock SUBSCRIBERS mutex: {}", e)))?;

    if subscribers.contains_key(name) {
        return Err(
            PyValueError::new_err(format!("Subscriber with name '{}' already exists", name)));
    }

    let node = NodeBuilder::new().create::<Service>()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to create node: {}", e)))?;

    let service = open_or_create_service(name, &node, &service_config.into())?;

    let subscriber = service
        .subscriber_builder()
        .buffer_size(subscriber_config.buffer_size())
        .create()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to create subscriber: {}", e)))?;

    subscribers.insert(name.to_string(), SafeSubscriber(subscriber));

    Ok(())
}

#[pyfunction]
pub fn destroy_subscriber(name: &str) -> PyResult<bool> {
    let mut subscribers = SUBSCRIBERS.lock()
        .map_err(|e|
            PyOSError::new_err(
                format!("Failed to lock SUBSCRIBERS mutex: {}", e)))?;

    match subscribers.remove(name) {
        None => Ok(false),
        Some(_) => Ok(true)
    }
}

#[pyfunction]
pub fn pop(
    name: &str
) -> PyResult<Option<Vec<u8>>> {
    let subscribers = SUBSCRIBERS.lock()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to lock SUBSCRIBERS mutex: {}", e)))?;

    let subscriber = unwrap_or_pyerr(
        subscribers.get(&name.to_string()),
        PyKeyError::new_err(format!("Subscriber not found {}", name)),
    )?;

    let sample = subscriber.0.receive()
        .map_err(|e| PyOSError::new_err(
            format!("Failed to receive data: {}", e)))?;

    match sample {
        Some(sample) => Ok(Some(sample.to_vec())),
        None => Ok(None)
    }
}
