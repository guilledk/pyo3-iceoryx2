from pyo3_iceoryx2._lowlevel import (
    create_publisher, push,
    create_subscriber, pop,
    create_notifier, create_listener, notify,
    timed_wait_all
)
from pyo3_iceoryx2.events import wait_event

PUB_CONNECTED = 0
SUB_CONNECTED = 1
SUB_READY = 2
NEW_DATA = 4

_publisher_config = {
    'initial_max_slice_len': 512,
    'allocation_strategy': 'power_of_two'
}

_subscriber_config = {
    'buffer_size': 1
}


class Pub0:
    def __init__(
        self, key: str,
        max_subs: int = 10,
        wait_subs: int = 1
    ):
        self._key = key
        self._feed_key = f'{key}-feed'
        self._event_key = f'{key}-event'

        self._feed_config = {
            'subscriber_max_buffer_size': 1,
            'max_subscribers': max_subs,
            'max_publishers': 1,
        }

        self._event_serv_config = {
            'max_notifiers': max_subs + 1,
            'max_listeners': max_subs + 1
        }

        create_publisher(self._feed_key, self._feed_config, _publisher_config)
        create_notifier(self._event_key, self._event_serv_config)
        create_listener(self._event_key, self._event_serv_config)
        self._total_subs = 0
        self._wait_subs = wait_subs

    @property
    def key(self) -> str:
        return self._key

    def _process_sub_events(self):
        events = []
        subs_ready = 0
        while self._total_subs < self._wait_subs or subs_ready < self._total_subs:
            events = timed_wait_all(self._event_key, 1000)
            self._total_subs += len([e for e in events if e == SUB_CONNECTED])
            subs_ready += sum([e for e in events if e == SUB_READY])
            notify(self._event_key, PUB_CONNECTED)

    def send(self, payload: bytes):
        self._process_sub_events()
        push(self._feed_key, payload)
        notify(self._event_key, NEW_DATA)


class Sub0:
    def __init__(self, key: str, max_subs: int = 10):
        self._key = key
        self._feed_key = f'{key}-feed'
        self._event_key = f'{key}-event'

        self._feed_config = {
            'subscriber_max_buffer_size': 1,
            'max_subscribers': max_subs,
            'max_publishers': 1,
        }

        self._event_serv_config = {
            'max_notifiers': max_subs + 1,
            'max_listeners': max_subs + 1
        }

        create_subscriber(self._feed_key, self._feed_config, _publisher_config)
        create_notifier(self._event_key, self._event_serv_config)
        create_listener(self._event_key, self._event_serv_config)

    @property
    def key(self) -> str:
        return self._key

    def subscribe(self):
        events = []
        while PUB_CONNECTED not in events:
            events = timed_wait_all(self._event_key, 1000)
            notify(self._event_key, SUB_CONNECTED)

    def recv(self) -> bytes:
        notify(self._event_key, SUB_READY)
        wait_event(self._event_key, NEW_DATA)
        msg = pop(self._feed_key)
        return msg


