use iceoryx2::port::publisher::Publisher;
use iceoryx2::port::subscriber::Subscriber;
use iceoryx2::prelude::ipc::Service;
use iceoryx2::prelude::{AttributeVerifier, Node, NodeBuilder, ServiceName};
use iceoryx2::service::port_factory::publish_subscribe::PortFactory;
use pyo3::exceptions::{PyKeyError, PyOSError, PyValueError};
use pyo3::prelude::*;
use crate::proxies::{PyPublisherConfig, PyServiceConfig, PySubscriberConfig};


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

#[pyclass(name = "Publisher")]
pub struct PyPublisher {
    name: String,
    service_config: PyServiceConfig,
    publisher_config: PyPublisherConfig,
    inner: Option<Publisher<Service, [u8], ()>>
}

unsafe impl Send for PyPublisher {}
unsafe impl Sync for PyPublisher {}

#[pymethods]
impl PyPublisher {
    #[new]
    #[pyo3(signature = (name, service_config=None, publisher_config=None))]
    fn new(
        name: &str,
        service_config: Option<PyServiceConfig>,
        publisher_config: Option<PyPublisherConfig>,
    ) -> Self {
        PyPublisher {
            name: name.to_string(),
            service_config: service_config.unwrap_or_default(),
            publisher_config: publisher_config.unwrap_or_default(),
            inner: None
        }
    }

    fn create(&mut self) -> PyResult<()> {
        let node = NodeBuilder::new().create::<Service>()
            .map_err(|e| PyOSError::new_err(
                format!("Failed to create node: {}", e)))?;

        let service = open_or_create_service(
            &self.name, &node, &self.service_config.clone().into())?;

        self.inner = Some(service
            .publisher_builder()
            .initial_max_slice_len(self.publisher_config.initial_max_slice_len())
            .allocation_strategy(self.publisher_config.allocation_strategy()?)
            .unable_to_deliver_strategy(self.publisher_config.unable_to_deliver_strategy()?)
            .max_loaned_samples(self.publisher_config.max_loaned_samples())
            .create()
            .map_err(|e| PyOSError::new_err(
                format!("Failed to create publisher: {}", e)))?);

        Ok(())
    }

    fn push(&mut self, data: &[u8]) -> PyResult<usize> {
        let publisher = match &self.inner {
            Some(publisher) => Ok(publisher),
            None => Err(PyValueError::new_err("Publisher not created yet"))
        }?;

        let slice = publisher.loan_slice_uninit(data.len())
            .map_err(|e| PyOSError::new_err(
                format!("Failed to loan slice: {}", e)))?;

        let slice = slice.write_from_slice(data);
        Ok(slice.send()
            .map_err(
                |e| PyKeyError::new_err(
                    format!("Failed to send data: {}", e)))?)
    }
}


#[pyclass(name = "Subscriber")]
pub struct PySubscriber {
    name: String,
    service_config: PyServiceConfig,
    subscriber_config: PySubscriberConfig,
    inner: Option<Subscriber<Service, [u8], ()>>
}

unsafe impl Send for PySubscriber {}
unsafe impl Sync for PySubscriber {}

#[pymethods]
impl PySubscriber {
    #[new]
    #[pyo3(signature = (name, service_config=None, subscriber_config=None))]
    fn new(
        name: &str,
        service_config: Option<PyServiceConfig>,
        subscriber_config: Option<PySubscriberConfig>,
    ) -> Self {
        PySubscriber {
            name: name.to_string(),
            service_config: service_config.unwrap_or_default(),
            subscriber_config: subscriber_config.unwrap_or_default(),
            inner: None
        }
    }

    fn create(&mut self) -> PyResult<()> {
        let node = NodeBuilder::new().create::<Service>()
            .map_err(|e| PyOSError::new_err(
                format!("Failed to create node: {}", e)))?;

        let service = open_or_create_service(
            &self.name, &node, &self.service_config.clone().into())?;

        self.inner = Some(service
            .subscriber_builder()
            .buffer_size(self.subscriber_config.buffer_size())
            .create()
            .map_err(|e| PyOSError::new_err(
                format!("Failed to create subscriber: {}", e)))?);

        Ok(())
    }

    pub fn pop(&self) -> PyResult<Option<Vec<u8>>> {
        let subscriber = match &self.inner {
            Some(publisher) => Ok(publisher),
            None => Err(PyValueError::new_err("Publisher not created yet"))
        }?;

        let sample = subscriber.receive()
            .map_err(|e| PyOSError::new_err(
                format!("Failed to receive data: {}", e)))?;

        match sample {
            Some(sample) => Ok(Some(sample.to_vec())),
            None => Ok(None)
        }
    }
}