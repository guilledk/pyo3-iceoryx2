import time
from pyo3_iceoryx2 import (
    create_publisher, push,
    create_notifier, create_listener, notify, timed_wait_all
)

feed_channel = 'test_feed'
event_channel = 'test_events'

feed_config = {
    'subscriber_max_buffer_size': 1,
    'max_subscribers': 1,
    'max_listeners': 1
}

publisher_config = {
    'initial_max_slice_len': 512,
    'unable_to_deliver_strategy': 'block'
}

event_serv_config = {
    'max_notifiers': 2,
    'max_listeners': 2
}

create_publisher(feed_channel, feed_config, publisher_config)
create_notifier(event_channel, event_serv_config)
create_listener(event_channel, event_serv_config)

READY = 0
NEW_DATA = 1

amount = 100_000

start_time = time.time()
try:
    for i in range(amount):
       push(feed_channel, f'iteration {i}'.encode('utf-8'))
       notify(event_channel, NEW_DATA)
       events = []
       while READY not in events:
           events = timed_wait_all(event_channel, 1000)

except KeyboardInterrupt:
    print("interrupted")

end_time = time.time()

elapsed = end_time - start_time
speed = int(amount / elapsed)

print(f'elapsed {elapsed:.2f} seconds, {speed} items/sec')