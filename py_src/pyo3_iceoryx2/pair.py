from pyo3_iceoryx2._lowlevel import (
    Publisher, Subscriber,
    Notifier, Listener
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
        self._pub = Publisher(self._feed_key, _feed_config, _publisher_config)
        self._sub = Subscriber(self._feed_key, _feed_config, _subscriber_config)
        self._notifier = Notifier(self._event_key, _event_serv_config)
        self._listener = Listener(self._event_key, _event_serv_config)

    @property
    def key(self) -> str:
        return self._key

    def connect(self):
        self._pub.create()
        self._sub.create()
        self._notifier.create()
        self._listener.create()
        events = self._listener.wait_events(
            (self.PEER_CONNECT_EVENT, self.PEER_DISCONNECT_EVENT,),
            1000,
            lambda: self._notifier.notify(self.CONNECT_EVENT)
        )

        if self.PEER_DISCONNECT_EVENT in events:
            raise ValueError(f'Unexpected Pair disconnect event!')

    def disconnect(self):
        self._listener.wait_event(
            self.PEER_DISCONNECT_EVENT,
            1000,
            lambda: self._notifier.notify(self.DISCONNECT_EVENT)
        )

    def send(self, payload: bytes):
        self._pub.push(payload)
        self._notifier.notify(self.NEW_DATA)
        self._listener.wait_event(
            self.PEER_READY,
            1000,
            None
        )

    def recv(self) -> bytes:
        self._listener.wait_event(
            self.PEER_NEW_DATA,
            1000,
            None
        )
        msg = self._sub.pop()
        self._notifier.notify(self.READY)
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
