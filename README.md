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

## Input Methods

The tool supports multiple ways to provide FSQL commands, with automatic precedence handling:

### Interactive REPL (default)
When no other input is provided, the tool starts an interactive REPL:

```shell
$ fsqlctl eyJ...lA
```

### Command Line Input
Execute commands directly from the command line using `-c` or `--command`:

```shell
$ fsqlctl eyJ...lA -c "QUERY module_activity.** WITH module_activity.activity_id = LOAD"
```

### File Input
Read commands from a file using `-f` or `--file`:

```shell
$ fsqlctl eyJ...lA -f query.txt
```

### Piped Input
Pipe queries to the command (automatically detected):

```shell
echo "QUERY module_activity.** WITH module_activity.activity_id = LOAD AND module_activity.actor.process.file.name = 'regsvr32.exe' AFTER 1h" | fsqlctl eyJ...lA | jq
```

**Note:** The `-c` and `-f` options are mutually exclusive - you cannot specify both at the same time.

**Input Precedence:** Command line (`-c`) or File (`-f`) > Piped input > REPL

## Release Builds

To generate a binary without debug symbols:

```shell
$ cargo build --release
```
