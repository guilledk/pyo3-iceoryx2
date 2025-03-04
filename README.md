# pyo3-iceoryx2

Python bindings to [iceoryx2](https://docs.rs/iceoryx2/latest/iceoryx2/) (from their README):

"""

Welcome to iceoryx2, the efficient, and ultra-low latency inter-process communication middleware. This library is designed to provide you with fast and reliable zero-copy and lock-free inter-process communication mechanisms.

So if you want to communicate efficiently between multiple processes or applications iceoryx2 is for you. With iceoryx2, you can:

- Send huge amounts of data using a publish/subscribe, request/response (planned), pipeline (planned) or blackboard pattern (planned), making it ideal for scenarios where large datasets need to be shared.
- Exchange signals through events, enabling quick and reliable signaling between processes.

iceoryx2 is based on a service-oriented architecture (SOA) and facilitates seamless inter-process communication (IPC).

"""

## Motivation

Needed fast IPC message passing for GIL-less threading on python, and didn't want to wait for `3.13t`, running multiple CPU bound tasks in different python processes and communicating with this seemed like an aproach worth exploring.

## Installation

### Recomended:
On each tag push to this repo a workflow is run that build wheels and publishes them under a new release.

Check [releases]() page and fetch the link to the latest wheel then install it using your prefered package manager.

### Build from source:

Requirements:

- [rust toolchain](https://rustup.rs/)
- [maturin](https://github.com/PyO3/maturin)

#### For NixOS users:
If using `nixos` use the `nix-shell` provided:

    nix-shell --run 'uv run maturin dev'

## specifics

*This is a work in progress and waiting on feedback from `iceoryx2` creators.*

### pyo3_iceoryx2._lowlevel:

- `Publisher`: Wraps `iceoryx2::port::publisher::Publisher`, offers creation and push message
- `Subscriber`: Wraps `iceoryx2::port::subscriber::Subscriber`, offers creation and pop message
- `Notifier`: Wraps `iceoryx2::port::notifier::Notifier`, offers creation and notify event
- `Listener`: Wraps `iceoryx2::port::notifier::Listener`, offers creation and event wait apis

IMPORTANT: So far can only send `bytes` as im using `<ipc::Service, [u8], ()>` as types for `Publisher` and `Subscriber`.

### pyo3_iceoryx2.pair

Implemented `nng`'s [Pair0](https://pynng.readthedocs.io/en/latest/core.html#pynng.Pair0) and [Pair1](https://pynng.readthedocs.io/en/latest/core.html#pynng.Pair1)

This is a bidirectional channel where all messages are guaranteed to reach peer in order.

See example at `examples/pair/main.py`

### pyo3_iceoryx2.pubsub

(IMPORTANT: caution using this, needs better impl)

Implemented `nng`'s [Pub0](https://pynng.readthedocs.io/en/latest/core.html#pynng.Pub0) and [Sub0](https://pynng.readthedocs.io/en/latest/core.html#pynng.Sub0)

This is a one directional one to many channel where all messages are guaranteed to reach all subscribers in order.