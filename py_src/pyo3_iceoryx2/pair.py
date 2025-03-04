from pyo3_iceoryx2.events import wait_event, wait_events
from pyo3_iceoryx2._lowlevel import (
    create_publisher, push, destroy_publisher,
    create_subscriber, pop, destroy_subscriber,
    create_notifier, notify, destroy_notifier,
    create_listener, destroy_listener
)


_feed_config = {
    'subscriber_max_buffer_size': 1,
    'max_subscribers': 2,
    'max_publishers': 2,
}

_publisher_config = {
    'initial_max_slice_len': 512,
    'allocation_strategy': 'power_of_two',
    'unable_to_deliver_strategy': 'block'
}

_subscriber_config = {
    'buffer_size': 1
}

_event_serv_config = {
    'max_notifiers': 2,
    'max_listeners': 2
}


class BasePair:
    CONNECT_EVENT: int
    DISCONNECT_EVENT: int
    PEER_CONNECT_EVENT: int
    PEER_DISCONNECT_EVENT: int
    READY: int
    PEER_READY: int
    NEW_DATA: int
    PEER_NEW_DATA: int

    def __init__(self, key: str):
        self._key = key
        self._feed_key = f'{key}-feed'
        self._event_key = f'{key}-event'

    @property
    def key(self) -> str:
        return self._key

    def connect(self):
        create_publisher(self._feed_key, _feed_config, _publisher_config)
        create_subscriber(self._feed_key, _feed_config, _subscriber_config)
        create_notifier(self._event_key, _event_serv_config)
        create_listener(self._event_key, _event_serv_config)

        events = wait_events(
            self._event_key,
            (self.PEER_CONNECT_EVENT, self.PEER_DISCONNECT_EVENT),
            call_back=lambda: notify(self._event_key, self.CONNECT_EVENT)
        )

        if self.PEER_DISCONNECT_EVENT in events:
            raise ValueError(f'Unexpected Pair disconnect event!')

    def disconnect(self):
        wait_events(
            self._event_key,
            (self.PEER_DISCONNECT_EVENT,),
            call_back=lambda: notify(self._event_key, self.DISCONNECT_EVENT)
        )
        destroy_listener(self._event_key)
        destroy_notifier(self._event_key)
        destroy_subscriber(self._feed_key)
        destroy_publisher(self._feed_key)

    def send(self, payload: bytes):
        push(self._feed_key, payload)
        notify(self._event_key, self.NEW_DATA)
        wait_event(self._event_key, self.PEER_READY)

    def recv(self) -> bytes:
        wait_event(self._event_key, self.PEER_NEW_DATA)
        msg = pop(self._feed_key)
        notify(self._event_key, self.READY)
        return msg

    def __enter__(self):
        self.connect()
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self.disconnect()
        return False


class Pair0(BasePair):
    CONNECT_EVENT = 0
    DISCONNECT_EVENT = 1
    PEER_CONNECT_EVENT = 2
    PEER_DISCONNECT_EVENT = 3
    READY = 4
    PEER_READY = 5
    NEW_DATA = 6
    PEER_NEW_DATA = 7


class Pair1(BasePair):
    CONNECT_EVENT = 2
    DISCONNECT_EVENT = 3
    PEER_CONNECT_EVENT = 0
    PEER_DISCONNECT_EVENT = 1
    READY = 5
    PEER_READY = 4
    NEW_DATA = 7
    PEER_NEW_DATA = 6
