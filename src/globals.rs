use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use iceoryx2::port::listener::Listener;
use iceoryx2::port::notifier::Notifier;
use iceoryx2::port::publisher::Publisher;
use iceoryx2::port::subscriber::Subscriber;
use iceoryx2::prelude::ipc::Service;


pub(crate) struct SafePublisher(pub Publisher<Service, [u8], ()>);
unsafe impl Sync for SafePublisher {}
unsafe impl Send for SafePublisher {}

pub(crate) struct SafeSubscriber(pub Subscriber<Service, [u8], ()>);
unsafe impl Sync for SafeSubscriber {}
unsafe impl Send for SafeSubscriber {}

pub(crate) struct SafeNotifier(pub Notifier<Service>);
unsafe impl Sync for SafeNotifier {}
unsafe impl Send for SafeNotifier {}

#[derive(Debug)]
pub(crate) struct SafeListener(pub Listener<Service>);
unsafe impl Sync for SafeListener {}
unsafe impl Send for SafeListener {}


pub(crate) static PUBLISHERS: LazyLock<Mutex<HashMap<String, SafePublisher>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

pub(crate) static SUBSCRIBERS: LazyLock<Mutex<HashMap<String, SafeSubscriber>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

pub(crate) static NOTIFIERS: LazyLock<Mutex<HashMap<String, SafeNotifier>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

pub(crate) static LISTENERS: LazyLock<Mutex<HashMap<String, SafeListener>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});
