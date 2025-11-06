# FSQL Command Line

This project provies a command line tool for working with the FSQL API.

## Prerequisites
This project requires the rust toolchain, including cargo.

## Getting Started

Launching the application is as simple as passing the token:

```shell
$ cargo run eyJ...lA
```

Where "eyJ...lA" is the API bearer token. If you're using a compiled version, use ``fsqlctl`` instead:

```shell
$ fsqlctl eyJ...lA
```

You can also pipe queries to the command and then to another process such as ``jq``:

```shell
echo "QUERY module_activity.** WITH module_activity.activity_id = LOAD AND module_activity.actor.process.file.name = 'regsvr32.exe' AFTER 1h" | fsqlctl eyJ...lA | jq
```

## Release Builds

To generate a binary without debug symbols:

```shell
$ cargo build --release
```
