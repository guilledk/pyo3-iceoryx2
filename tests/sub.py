import time
from pyo3_iceoryx2 import (
    create_subscriber, pop,
    create_notifier, create_listener, notify, timed_wait_all
)

feed_channel = 'test_feed'
in_event_channel = 'test_events_0'
out_event_channel = 'test_events_1'

create_subscriber(feed_channel)
create_notifier(out_event_channel)
create_listener(in_event_channel)

READY = 0
NEW_DATA = 1

try:
    while True:
        events = []
        while NEW_DATA not in events:
            events = timed_wait_all(in_event_channel, 1000)

        msg = pop(feed_channel)
        print(msg)

        notify(out_event_channel, READY)

except KeyboardInterrupt:
    print("interrupted")
