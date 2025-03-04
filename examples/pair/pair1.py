from pyo3_iceoryx2 import Pair1

pair = Pair1('example-pair')
pair.connect()

# first message is total number of messages to be streamed
amount = int(pair.recv().decode('utf-8'))

# finally receive
try:
    for i in range(amount):
        msg = pair.recv()
        print(msg[:16])

    pair.send(b'END')

except KeyboardInterrupt:
    print("interrupted")

