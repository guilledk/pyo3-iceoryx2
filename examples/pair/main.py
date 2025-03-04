import time
import multiprocessing as mp

from pair0 import pair0_main
from pair1 import pair1_main


if __name__ == '__main__':
    amount = 10_000

    pair0 = mp.Process(target=pair0_main, args=(amount,))
    pair0.start()

    pair1 = mp.Process(target=pair1_main, args=(amount,))
    pair1.start()
    pair1.join()

    pair0.join()
