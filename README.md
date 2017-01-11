# bus1-rs

Wrapper around the bus1 IPC for Linux.

This project is unstable and will change to match upstream bus1 changes.

The currently exposed API is incomplete and probably not safe.

## bus1 setup

Tested inside a Fedora 24 virtual machine with a 4.8 Linux kernel.

```
$ git clone https://github.com/bus1-rust/bus1.git
$ git checkout fix-local-handle-id
$ cd bus1
$ make
$ sudo insmod ipc/bus1/bus1.ko
```

## How to build, test, generate the documentation

```
$ cargo build
$ cargo test
$ cargo doc
```

## Example

```
#[macro_use] extern crate log;
extern crate env_logger;
extern crate bus1;

use bus1::*;

fn main() {
    env_logger::init().unwrap();

    let peer3 = Peer::new().unwrap();
    let peer4 = Peer::new().unwrap();
    let peer5 = Peer::new().unwrap();

    let node1 = Handle::new(4);

    let peer4_node1 = match peer3.send_handle_to_peer(&node1, &peer4) {
        Err(e) => panic!("{}", e),
        Ok(h) => h,
    };

    let mut msg = MessageBuilder::new();
    msg.add_destinations(&mut vec![peer4_node1]);

    match peer4.send(msg) {
        Err(e) => panic!("{}", e),
        _ => (),
    }

    let message = match peer3.recv() {
        Err(e) => panic!("{}", e),
        Ok(msg) => msg
    };

    Message::release_slice(message);

    let node2 = Handle::new(8);

    let peer5_node2 = match peer3.send_handle_to_peer(&node2, &peer5) {
        Err(e) => panic!("{}", e),
        Ok(h) => h,
    };

    let mut msg = MessageBuilder::new();
    msg.add_destinations(&mut vec![peer5_node2]);

    match peer5.send(msg) {
        Err(e) => panic!("{}", e),
        _ => (),
    }

    let message = match peer3.recv() {
        Err(e) => panic!("{}", e),
        Ok(msg) => msg
    };

    Message::release_slice(message);
}

```

```
$ RUST_LOG="bus1=debug" cargo run
```

## License

Licensed under the MIT license, see LICENSE.
