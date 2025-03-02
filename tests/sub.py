import time

from pyo3_iceoryx2 import (
    notify,
    timed_wait_one
)

from common import (
    event_channel,
    init_subscriber,
    receive,
    PUB_CONNECTED,
    SUB_CONNECTED
)


# setup coms
init_subscriber()

# wait until other side is up
event = None
while event != PUB_CONNECTED:
    # wait for a publisher to be ready for 1000 ms
    event = timed_wait_one(event_channel, 1000)
    # notify sub readiness
    notify(event_channel, SUB_CONNECTED)  # keep broadcasting event in case

# first message is total number of messages to be streamed
amount = int(receive().decode('utf-8'))

# finally receive
try:
    for i in range(amount):
        msg = receive()
        print(msg[:16])

except KeyboardInterrupt:
    print("interrupted")
