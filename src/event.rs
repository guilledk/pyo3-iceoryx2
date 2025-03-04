use std::time::Duration;
use iceoryx2::node::{Node, NodeBuilder};
use iceoryx2::port::listener::Listener;
use iceoryx2::port::notifier::Notifier;
use iceoryx2::prelude::ipc::Service;
use iceoryx2::prelude::{AttributeVerifier, EventId, ServiceName};
use iceoryx2::service::port_factory::event::PortFactory;
use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};
use pyo3::exceptions::{PyOSError, PyValueError};
use crate::proxies::{PyServiceConfig};


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


#[pyclass(name = "Notifier")]
pub struct PyNotifier {
    name: String,
    service_config: PyServiceConfig,
    inner: Option<Notifier<Service>>,
}

unsafe impl Send for PyNotifier {}
unsafe impl Sync for PyNotifier {}

#[pymethods]
impl PyNotifier {
    #[new]
    #[pyo3(signature = (name, service_config=None))]
    fn new(
        name: &str,
        service_config: Option<PyServiceConfig>,
    ) -> Self {
        PyNotifier {
            name: name.to_string(),
            service_config: service_config.unwrap_or_default(),
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
            .notifier_builder()
            .create()
            .map_err(|e| PyOSError::new_err(
                format!("Could not create notification service {}: {}", self.name, e)))?);

        Ok(())
    }

    pub fn notify(
        &self,
        event_id: usize
    ) -> PyResult<()> {
        let notifier = match &self.inner {
            Some(notifier) => Ok(notifier),
            None => Err(PyValueError::new_err("Notifier not created yet"))
        }?;

        notifier.notify_with_custom_event_id(EventId::new(event_id))
            .map_err(|e| PyOSError::new_err(
                format!("Failed to notfiy event: {}", e)))?;

        Ok(())
    }
}

#[pyclass(name = "Listener")]
pub struct PyListener {
    name: String,
    service_config: PyServiceConfig,
    inner: Option<Listener<Service>>,
}

unsafe impl Send for PyListener {}
unsafe impl Sync for PyListener {}

#[pymethods]
impl PyListener {
    #[new]
    #[pyo3(signature = (name, service_config=None))]
    fn new(
        name: &str,
        service_config: Option<PyServiceConfig>,
    ) -> Self {
        PyListener {
            name: name.to_string(),
            service_config: service_config.unwrap_or_default(),
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
            .listener_builder()
            .create()
            .map_err(|e| PyOSError::new_err(
                format!("Could not create notification service {}: {}", self.name, e)))?);

        Ok(())
    }

    pub fn timed_wait_one(
        &self,
        timeout: u64,
    ) -> PyResult<Option<usize>> {
        let listener = match &self.inner {
            Some(listener) => Ok(listener),
            None => Err(PyValueError::new_err("Listener not created yet"))
        }?;

        let event = listener.timed_wait_one(Duration::from_millis(timeout))
            .map_err(|e| PyOSError::new_err(
                format!("Timed wait all event failed: {}", e)))?
            .map(|event| event.as_value());

        Ok(event)
    }

    pub fn timed_wait_all(
        &self,
        timeout: u64,
    ) -> PyResult<Vec<usize>> {
        let listener = match &self.inner {
            Some(listener) => Ok(listener),
            None => Err(PyValueError::new_err("Listener not created yet"))
        }?;

        let mut events = Vec::new();
        let collect_event = |e: EventId| events.push(e.as_value());

        listener.timed_wait_all(collect_event, Duration::from_millis(timeout))
            .map_err(|e| PyOSError::new_err(
                format!("Timed wait all event failed: {}", e)))?;

        Ok(events)
    }

    pub fn wait_event(
        &self,
        py: Python,
        event: usize,
        timeout: u64,
        call_back: PyObject,
    ) -> PyResult<()> {
        let mut found_event = false;
        while !found_event {
            if !call_back.is_none(py) {
                call_back.call0(py)?;
            }

            let next_event = match self.timed_wait_one(timeout)? {
                Some(event) => event,
                None => continue
            };
            found_event = next_event == event;
        }
        Ok(())
    }

    pub fn wait_events(
        &self,
        py: Python,
        events: Vec<usize>,
        timeout: u64,
        call_back: PyObject,
    ) -> PyResult<Vec<usize>> {
        let mut found_event = false;
        let mut results = Vec::new();
        while !found_event {
            if !call_back.is_none(py) {
                call_back.call0(py)?;
            }

            let next_events = self.timed_wait_all(timeout)?;
            for next_event in &next_events {
                found_event = events.contains(next_event);
                if found_event {
                    results = next_events;
                    break;
                }
            }
        }
        Ok(results)
    }
}