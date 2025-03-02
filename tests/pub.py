import time
import os
import random

from pyo3_iceoryx2 import (
    notify,
    timed_wait_one
)

from common import (
    event_channel,
    init_publisher,
    send,
    generate_random_messages,
    PUB_CONNECTED,
    SUB_CONNECTED

)


# setup comms
init_publisher()

# wait until other side is up
event = None
while event != SUB_CONNECTED:
    # wait for a subscriber to be ready for 1000 ms
    event = timed_wait_one(event_channel, 1000)
    # notify sub about publisher readiness
    notify(event_channel, PUB_CONNECTED)  # keep broadcasting event in case

# send total msg amount encoded as string as first msg
amount = 100_000
send(str(amount).encode('utf-8'))

# generate {amount} msgs of len random(256b, 20kb)
messages = generate_random_messages(
    amount, min_msg_len=256, max_msg_len=20 * 1024)

# send messages
start_time = time.time()
try:
    for msg in messages:
        send(msg)

except KeyboardInterrupt:
    print("interrupted")

# calculate runtime stats
end_time = time.time()

elapsed = end_time - start_time
speed = int(amount / elapsed)

print(f'elapsed {elapsed:.2f} seconds, {speed} items/sec')