import time
import argparse

from pyo3_iceoryx2 import Pair0
from pyo3_iceoryx2._testing import generate_random_messages


def pair0_main(amount: int):
    # generate random payloads of 256 to 20kb
    messages, total_len = generate_random_messages(
        amount, min_msg_len=256, max_msg_len=20 * 1024)

    with Pair0('example-pair') as pair:
        start_time = time.time()

        # send messages
        for msg in messages:
            pair.send(msg)

        # calculate runtime stats
        end_time = time.time()

        elapsed = end_time - start_time
        speed = int(amount / elapsed)

        print(f'elapsed {elapsed:.2f} seconds, {speed} items/sec')

    print(f'total bytes transmitted: {total_len:,}')


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("amount", type=int, help="Number of msgs to send")
    args = parser.parse_args()

    pair0_main(args.amount)
