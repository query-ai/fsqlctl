# FSQL Command Line

This project provies a command line tool for working with the FSQL API.

## Prerequisites
This project requires the rust toolchain, including cargo.

## Getting Started

Launching the application is as simple as passing the token:

```shell
$ cargo run eyJ...lA
```

Where "eyJ...lA" is the API bearer token

## Release Builds

To generate a binary without debug symbols:

```shell
$ cargo build --release
```
