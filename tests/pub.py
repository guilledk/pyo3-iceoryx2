import time
from pyo3_iceoryx2 import (
    create_publisher, push,
    create_notifier, create_listener, notify, timed_wait_all
)

feed_channel = 'test_feed'
in_event_channel = 'test_events_0'
out_event_channel = 'test_events_1'

create_publisher(feed_channel)
create_notifier(in_event_channel)
create_listener(out_event_channel)

READY = 0
NEW_DATA = 1

amount = 100_000

start_time = time.time()
try:
    for i in range(amount):
       push(feed_channel, f'iteration {i}'.encode('utf-8'))
       notify(in_event_channel, NEW_DATA)
       events = []
       while READY not in events:
           events = timed_wait_all(out_event_channel, 1000)

except KeyboardInterrupt:
    print("interrupted")

end_time = time.time()

elapsed = end_time - start_time
speed = int(amount / elapsed)

print(f'elapsed {elapsed:.2f} seconds, {speed} items/sec')