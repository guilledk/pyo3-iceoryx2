import time
import os
import random

from pyo3_iceoryx2 import Pair0

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


amount = 100_000

# generate {amount} msgs of len random(256b, 20kb)
messages = generate_random_messages(
    amount, min_msg_len=256, max_msg_len=20 * 1024)

pair = Pair0('example-pair')
pair.connect()
pair.send(str(amount).encode('utf-8'))

# send messages
start_time = time.time()
try:
    for msg in messages:
        pair.send(msg)

except KeyboardInterrupt:
    print("interrupted")

# calculate runtime stats
end_time = time.time()

elapsed = end_time - start_time
speed = int(amount / elapsed)

print(f'elapsed {elapsed:.2f} seconds, {speed} items/sec')

pair.recv()