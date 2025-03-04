import argparse
from pyo3_iceoryx2 import Pair1

def pair1_main(amount: int):
    with Pair1('example-pair') as pair:
        for i in range(amount):
            msg = pair.recv()
            print(msg[:16])

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("amount", type=int, help="Number of msgs to send")
    args = parser.parse_args()

    pair1_main(args.amount)
