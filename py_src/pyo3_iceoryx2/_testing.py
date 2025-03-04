import os
import random


def generate_random_messages(amount: int, min_msg_len: int, max_msg_len: int) -> tuple[list[bytes], int]:
    messages = []
    total_len = 0
    for i in range(amount):
        msg = f'iteration: {i} raw: '.encode('utf-8')
        msg += os.urandom(random.randint(min_msg_len, max_msg_len))
        total_len += len(msg)
        messages.append(msg)

    return messages, total_len
