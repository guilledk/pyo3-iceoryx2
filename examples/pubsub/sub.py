import argparse

from pyo3_iceoryx2 import Sub0

def subscriber_main(amount: int):
    sub = Sub0('example-pubsub')
    sub.subscribe()

    messages = []

    # finally receive
    try:
        exit = False
        while not exit:
            msg = sub.recv()
            messages.append(msg)
            msg_num = int(msg.decode('utf-8'))
            exit = msg_num == amount - 1
            print(msg_num)


    except KeyboardInterrupt:
        print("interrupted")

    sub.unsubscribe()

    print(f'received {len(messages)} messages')

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run the subscriber")
    parser.add_argument("amount", type=int, help="Number of msgs to send")
    args = parser.parse_args()

    subscriber_main(args.amount)
