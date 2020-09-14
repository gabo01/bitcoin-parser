# Bitcoin Parser

[![Lines of code][l1]][l2]

[l1]: https://tokei.rs/b1/github/gabo01/bitcoin-parser
[l2]: https://github.com/gabo01/bitcoin-parser

Bitcoin parser is an application and library for performing analysis on the bitcoin blockchain.
It consists on a CLI used to perform some standard blockchain operations and a library used which
can be used to develop custom applications.

### Prerequisites

In order to compile the library, you will need to have:

- Rust version 1.46.0 or later
- Cargo verion 1.46.0 or later

### Building from Source

For the debug version run:

```bash
cargo build
```

For the release version, which applies optimizations, run:

```bash
cargo build --release
```

## Features

- [x] Struct model for the blockchain
- [ ] Parser for blk.dat files
- [ ] Transaction graph construction
- [ ] Address graph construction
- [ ] Script analysis

**Disclaimer:** The features checked are not stable yet and won't be as long as the application
version does not reach 1.0.0
