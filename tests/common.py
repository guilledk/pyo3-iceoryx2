import os
import random

from pyo3_iceoryx2 import (
    create_publisher, push,
    create_subscriber, pop,
    create_notifier, create_listener, notify,
    timed_wait_all, timed_wait_one
)

PUB_CONNECTED = 0
SUB_CONNECTED = 1
READY = 2
NEW_DATA = 3

feed_channel = 'test_feed'
event_channel = 'test_events'

feed_config = {
    'subscriber_max_buffer_size': 1,
    'max_subscribers': 1,
    'max_listeners': 1
}

publisher_config = {
    'initial_max_slice_len': 512,
    'allocation_strategy': 'power_of_two'
}

subscriber_config = {
    'buffer_size': 1
}

event_serv_config = {
    'max_notifiers': 2,
    'max_listeners': 2
}

def init_publisher():
    create_publisher(feed_channel, feed_config, publisher_config)
    create_notifier(event_channel, event_serv_config)
    create_listener(event_channel, event_serv_config)

def init_subscriber():
    create_subscriber(feed_channel, feed_config, subscriber_config)
    create_notifier(event_channel, event_serv_config)
    create_listener(event_channel, event_serv_config)

def send(msg: bytes):
    push(feed_channel, msg)
    notify(event_channel, NEW_DATA)
    events = []
    while READY not in timed_wait_all(event_channel, 1000):
        ...

def receive() -> bytes:
    while NEW_DATA not in timed_wait_all(event_channel, 1000):
        ...

    msg = pop(feed_channel)
    notify(event_channel, READY)
    return msg


def generate_random_messages(amount: int, min_msg_len: int, max_msg_len: int) -> list[bytes]:
    print(f"generating {amount} messages...")
    messages = []
    total_len = 0
    for i in range(amount):
        msg = f'iteration: {i} raw: '.encode('utf-8')
        msg += os.urandom(random.randint(min_msg_len, max_msg_len))
        total_len += len(msg)
        messages.append(msg)

    print(f'done, {total_len:,} bytes')
    return messages
