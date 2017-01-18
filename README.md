# bus1-rs

Wrapper around the bus1 IPC for Linux.

This project is unstable and will change to match upstream bus1 changes.

The currently exposed API is incomplete and probably not safe.

## bus1 setup

Tested inside a Fedora 24 virtual machine with a 4.8 Linux kernel.

```
$ git clone https://github.com/bus1/bus1.git
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

## License

Licensed under the terms of both the MIT license and the Apache License
(Version 2.0).
