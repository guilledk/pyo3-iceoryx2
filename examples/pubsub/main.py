import time
import multiprocessing as mp

from pyo3_iceoryx2 import Pub0, Sub0


def publisher_main(sub_amount: int):
    amount = 100_000

    pub = Pub0('example-pubsub', wait_subs=sub_amount)

    # send messages
    start_time = time.time()
    try:
        for i in range(amount):
            pub.send(str(i).encode('utf-8'))

    except KeyboardInterrupt:
        print("interrupted")

    # calculate runtime stats
    end_time = time.time()

    elapsed = end_time - start_time
    speed = int(amount / elapsed)

    print(f'elapsed {elapsed:.2f} seconds, {speed} items/sec')


def subscriber_main():
    sub = Sub0('example-pubsub')
    sub.subscribe()

    # finally receive
    try:
        exit = False
        while not exit:
            msg = sub.recv()
            msg_num = int(msg.decode('utf-8'))
            exit = msg_num == 99_999
            print(msg_num)


    except KeyboardInterrupt:
        print("interrupted")


if __name__ == '__main__':
    sub_amount = 2
    pub = mp.Process(target=publisher_main, args=(sub_amount,))
    pub.start()

    subs = []
    for _ in range(sub_amount):
        sub = mp.Process(target=subscriber_main)
        sub.start()
        subs.append(sub)

    for sub in subs:
        sub.join()

    pub.join()
