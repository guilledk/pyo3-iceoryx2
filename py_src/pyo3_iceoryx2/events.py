from pyo3_iceoryx2._lowlevel import (
    timed_wait_one
)

def wait_event(key: str, event: int, timeout: int = 1000):
    result = None
    while result != event:
        result = timed_wait_one(key, timeout)