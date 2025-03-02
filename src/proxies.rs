use iceoryx2::prelude::{AllocationStrategy, AttributeVerifier, UnableToDeliverStrategy};
use pyo3::{Bound, FromPyObject, PyAny, PyErr, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::types::PyDict;
use crate::py_struct;


py_struct!(
    PyServiceConfig {
        payload_alignment: Option<usize>,
        enable_safe_overflow: Option<bool>,
        subscriber_max_borrowed_samples: Option<usize>,
        history_size: Option<usize>,
        subscriber_max_buffer_size: Option<usize>,
        max_subscribers: Option<usize>,
        max_publishers: Option<usize>,
        max_nodes: Option<usize>,
    }
);

macro_rules! impl_into_attribute_verifier {
    ($struct_name:ident { $($field:ident),* $(,)? }) => {
        impl Into<AttributeVerifier> for $struct_name {
            fn into(self) -> AttributeVerifier {
                let mut spec = AttributeVerifier::default();
                $(
                    if let Some(value) = self.$field {
                        spec = spec.require(stringify!($field), &value.to_string());
                    }
                )*
                spec
            }
        }
    };
}

impl_into_attribute_verifier!(PyServiceConfig {
    payload_alignment,
    enable_safe_overflow,
    subscriber_max_borrowed_samples,
    history_size,
    subscriber_max_buffer_size,
    max_subscribers,
    max_publishers,
    max_nodes,
});

py_struct!(
    PyPublisherConfig {
        initial_max_slice_len: Option<usize>,
        allocation_strategy: Option<String>,
        unable_to_deliver_strategy: Option<String>,
        max_loaned_samples: Option<usize>,
    }
);


impl PyPublisherConfig {
    pub fn initial_max_slice_len(&self) -> usize {
        self.initial_max_slice_len.unwrap_or(512)
    }
    pub fn allocation_strategy(&self) -> PyResult<AllocationStrategy> {
        let strat = self.allocation_strategy.clone().unwrap_or("static".to_string());
        match strat.as_str() {
            "static" => Ok(AllocationStrategy::Static),
            "best_fit" => Ok(AllocationStrategy::BestFit),
            "power_of_two" => Ok(AllocationStrategy::PowerOfTwo),
            _ => Err(PyValueError::new_err(format!("Unknown alloc start {}", strat)))
        }
    }

    pub fn unable_to_deliver_strategy(&self) -> PyResult<UnableToDeliverStrategy> {
        let strat = self.unable_to_deliver_strategy.clone().unwrap_or("static".to_string());
        match strat.as_str() {
            "block" => Ok(UnableToDeliverStrategy::Block),
            "discard_sample" => Ok(UnableToDeliverStrategy::DiscardSample),
            _ => Err(PyValueError::new_err(format!("Unknown unable-to-deliver start {}", strat)))
        }
    }

    pub fn max_loaned_samples(&self) -> usize {
        self.max_loaned_samples.unwrap_or(10)
    }
}


py_struct!(
    PySubscriberConfig {
        buffer_size: Option<usize>
    }
);


impl PySubscriberConfig {
    pub fn buffer_size(&self) -> usize {
        self.buffer_size.unwrap_or(1)
    }
}
