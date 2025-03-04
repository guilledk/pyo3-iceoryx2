from typing import Callable

from pyo3_iceoryx2._lowlevel import (
    timed_wait_one, timed_wait_all
)


def wait_event(
    key: str,
    event: int,
    timeout: int = 1000,
    call_back: Callable | None = None
):
    result = None
    while result != event:
        result = timed_wait_one(key, timeout)
        if call_back:
            call_back()

def wait_events(
    key: str,
    events: list[int],
    timeout: int = 1000,
    call_back: Callable | None = None
) -> list[int]:
    results = []
    while len([e for e in results if e in events]) == 0:
        results = timed_wait_all(key, timeout)
        if call_back:
            call_back()

    return results