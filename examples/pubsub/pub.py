import time
import argparse
import multiprocessing as mp

from pyo3_iceoryx2 import Pub0


def publisher_main(msg_amount: int, sub_amount: int):

    pub = Pub0('example-pubsub', wait_subs=sub_amount)
    pub.connect()

    # send messages
    start_time = time.time()
    try:
        for i in range(msg_amount):
            pub.send(str(i).encode('utf-8'))

    except KeyboardInterrupt:
        print("interrupted")

    # calculate runtime stats
    end_time = time.time()

    pub.disconnect()

    elapsed = end_time - start_time
    speed = int(msg_amount / elapsed)

    print(f'elapsed {elapsed:.2f} seconds, {speed} items/sec')


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run the publisher")
    parser.add_argument("msg_amount", type=int, help="Number of msgs to send")
    parser.add_argument("sub_amount", type=int, help="Number of subscribers to wait for")
    args = parser.parse_args()

    publisher_main(args.msg_amount, args.sub_amount)