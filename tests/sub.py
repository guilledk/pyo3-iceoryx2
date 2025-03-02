import time
from pyo3_iceoryx2 import (
    create_subscriber, pop,
    create_notifier, create_listener, notify, timed_wait_all
)

feed_channel = 'test_feed'
event_channel = 'test_events'

feed_config = {
    'subscriber_max_buffer_size': 1,
    'max_subscribers': 1,
    'max_listeners': 1
}

subscriber_config = {}
subscriber_config = {
    'buffer_size': 1
}

event_serv_config = {
    'max_notifiers': 2,
    'max_listeners': 2
}

create_subscriber(feed_channel, feed_config, subscriber_config)
create_notifier(event_channel, event_serv_config)
create_listener(event_channel, event_serv_config)

READY = 0
NEW_DATA = 1

try:
    while True:
        events = []
        while NEW_DATA not in events:
            events = timed_wait_all(event_channel, 1000)

        msg = pop(feed_channel)
        print(msg)

        notify(event_channel, READY)

except KeyboardInterrupt:
    print("interrupted")
