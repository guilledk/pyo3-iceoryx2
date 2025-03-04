import time
import multiprocessing as mp

from pub import publisher_main
from sub import subscriber_main


if __name__ == '__main__':
    msg_amount = 10_000
    sub_amount = 2
    pub = mp.Process(target=publisher_main, args=(msg_amount, sub_amount,))
    pub.start()

    subs = []
    for _ in range(sub_amount):
        sub = mp.Process(target=subscriber_main, args=(msg_amount,))
        sub.start()
        subs.append(sub)

    for sub in subs:
        sub.join()

    pub.join()
