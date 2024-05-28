# Setup

`Setup` is a command line tool to automate and coordinate the execution of programs and scripts.
This tool only works on linux at the moment. Windows support may come sometimes in the future.

## Installing debian package

A debian package is currently only available for the amd64-architecture.

To install the debian package, download the .deb-file and run:

```bash
sudo dpkg -i setup.deb
```

## Installing with cargo

To install setup with cargo, clone this repository and run the following command in the root directory of this project:

```bash
cargo install --path .
```
