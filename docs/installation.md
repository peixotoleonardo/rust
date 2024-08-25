# Installation

We'll download Rust through rustup, a command line tool for
managing Rust versions and associated tools.

## Installing rustup on Linux

```bash
$ sudo apt-get install build-essential -y # to install GCC
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
## Troubleshooting

To verify whether you have Rust installed correctly, open
shell and enter this line:

```bash
$ rustc --version
```

## Updating and Uninstalling

```bash
$ rustup update         # to update Rust version
$ rustup self uninstall # to remove Rust
```