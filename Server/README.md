# Server code

This folder contains the code used to run the server for storing and accessing user and session data.

Server documentation is given in [the docs](server_docs.md).

## Usage

To start the server, run the following

```shell
cargo run --release
```

Not including the `--release` flag will result in no data being written to any files
