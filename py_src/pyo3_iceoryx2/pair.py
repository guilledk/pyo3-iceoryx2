from pyo3_iceoryx2._lowlevel import (
    create_publisher, push,
    create_subscriber, pop,
    create_notifier, create_listener, notify,
    timed_wait_one
)
from pyo3_iceoryx2.events import wait_event

PAIR0_CONNECTED = 0
PAIR1_CONNECTED = 1
PAIR0_READY = 2
PAIR1_READY = 3
PAIR0_NEW_DATA = 4
PAIR1_NEW_DATA = 5

_feed_config = {
    'subscriber_max_buffer_size': 1,
    'max_subscribers': 2,
    'max_publishers': 2,
}

_publisher_config = {
    'initial_max_slice_len': 512,
    'allocation_strategy': 'power_of_two'
}

_subscriber_config = {
    'buffer_size': 1
}

_event_serv_config = {
    'max_notifiers': 2,
    'max_listeners': 2
}


class Pair0:
    def __init__(self, key: str):
        self._key = key
        self._feed_key = f'{key}-feed'
        self._event_key = f'{key}-event'
        create_publisher(self._feed_key, _feed_config, _publisher_config)
        create_subscriber(self._feed_key, _feed_config, _subscriber_config)
        create_notifier(self._event_key, _event_serv_config)
        create_listener(self._event_key, _event_serv_config)

    @property
    def key(self) -> str:
        return self._key

    def connect(self):
        event = None
        while event != PAIR1_CONNECTED:
            event = timed_wait_one(self._event_key, 1000)
            notify(self._event_key, PAIR0_CONNECTED)

    def send(self, payload: bytes):
        push(self._feed_key, payload)
        notify(self._event_key, PAIR0_NEW_DATA)
        wait_event(self._event_key, PAIR1_READY)

    def recv(self) -> bytes:
        wait_event(self._event_key, PAIR1_NEW_DATA)
        msg = pop(self._feed_key)
        notify(self._event_key, PAIR0_READY)
        return msg


class Pair1:
    def __init__(self, key: str):
        self._key = key
        self._feed_key = f'{key}-feed'
        self._event_key = f'{key}-event'
        create_publisher(self._feed_key, _feed_config, _publisher_config)
        create_subscriber(self._feed_key, _feed_config, _subscriber_config)
        create_notifier(self._event_key, _event_serv_config)
        create_listener(self._event_key, _event_serv_config)

    @property
    def key(self) -> str:
        return self._key

    def connect(self):
        event = None
        while event != PAIR0_CONNECTED:
            event = timed_wait_one(self._event_key, 1000)
            notify(self._event_key, PAIR1_CONNECTED)

    def send(self, payload: bytes):
        push(self._feed_key, payload)
        notify(self._event_key, PAIR1_NEW_DATA)
        wait_event(self._event_key, PAIR0_READY)

    def recv(self) -> bytes:
        wait_event(self._event_key, PAIR0_NEW_DATA)
        msg = pop(self._feed_key)
        notify(self._event_key, PAIR1_READY)
        return msg
