from pyo3_iceoryx2._lowlevel import (
    Publisher, Subscriber,
    Notifier, Listener
)

PUB_CONNECTED = 0
SUB_CONNECTED = 1
SUB_DISCONNECTED = 2
SUB_READY = 3
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

        self._total_subs = 0
        self._wait_subs = wait_subs
        self._pub = Publisher(self._feed_key, self._feed_config, _publisher_config)
        self._notifier = Notifier(self._event_key, self._event_serv_config)
        self._listener = Listener(self._event_key, self._event_serv_config)

    @property
    def key(self) -> str:
        return self._key

    def create(self):
        self._pub.create()
        self._notifier.create()
        self._listener.create()

    def _process_sub_events(self):
        events = []
        subs_ready = 0
        while self._total_subs < self._wait_subs or subs_ready < self._total_subs:
            events = self._listener.timed_wait_all(1000)
            for event in events:
                if event == SUB_CONNECTED:
                    self._total_subs += 1
                elif event == SUB_DISCONNECTED:
                    self._total_subs -= 1
                elif event == SUB_READY:
                    subs_ready += 1
            self._notifier.notify(PUB_CONNECTED)

    def send(self, payload: bytes):
        self._process_sub_events()
        self._pub.push(payload)
        self._notifier.notify(NEW_DATA)


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
        self._sub = Subscriber(self._feed_key, self._feed_config, _publisher_config)
        self._notifier = Notifier(self._event_key, self._event_serv_config)
        self._listener = Listener(self._event_key, self._event_serv_config)

    @property
    def key(self) -> str:
        return self._key

    def subscribe(self):
        self._sub.create()
        self._notifier.create()
        self._listener.create()
        self._notifier.notify(SUB_CONNECTED)

    def unsubscribe(self):
        self._notifier.notify(SUB_DISCONNECTED)

    def recv(self) -> bytes:
        self._notifier.notify(SUB_READY)
        self._listener.wait_event(NEW_DATA, 1000, None)
        msg = self._sub.pop()
        return msg