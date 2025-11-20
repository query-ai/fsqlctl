# Changelog

## [0.15] (2025-11-20)

## [0.14] (2025-11-20)
- API token can be passed via environment variable ``FSQL_TOKEN``
- QAI-3663 - Added improved support for ``EXPLAIN CONNECTORS;``
- QAI-3662 - Added improved support for ``EXPLAIN ATTRIBUTES ...``

## [0.13] (2025-11-06)
- Replaced `atty` dependency with `std::io::IsTerminal`
- Setup documentation page

## [0.12] (2025-11-06)

- Added ``--command`` / ``-c`` to execute FSQL commands directly from CLI arguments
- Added ``--file`` / ``-f`` to read an FSQL command from a file
- Refactored command processing into shared `process_command` function

## [0.11] (2025-11-06)

- Removed `--stdin` / `-i` CLI flag (now we autodetect)
- Refactored stdin handling function that got too big and beefy

## [0.10] (2025-11-06)

- Several fixes to command output
- Proper structs for validate, explain, and query response types
- Added increased timeouts to the api client for long-running queries

## [0.9] (2025-11-06)

- Pretty print the expanded query in the repl
- Added a CommandResponse struct for parsing results more concretely

## [0.8] (2025-11-06)

- Added dependency on ``colored``library
- Added some colored terminal output to the REPL
- Split printing the welcome text, tips, and help into separate functions

## [0.7] (2025-11-06)

- Added new cli option ``-i`` / ``--stdin`` to read commands from a pipe
- When in piping mode, output is not pretty printed (pipe to ``jq`` for more fun)
- Split main.rs up since it was getting kinda big
- Messages from the api.rs module use the eprintln! macro so that piping output doesn't get muddled


## [0.6] (2025-10-30)

- Added command history support via readline

## [0.5] (2025-10-22)

- Reduce the number of keypresses required to end a multiline command
- Add support for terminating commands with the ; character

## [0.4] (2025-10-21)

- Fix github build action (was failing for aarm64)

## [0.3] (2025-10-21)

- Fix regular queries (previously only dispatched EXPLAIN queries)

## [0.2] (2025-10-20)

- Fix github actions for build/release

## [0.1] (2025-10-19)

- Initial REPL release
- Adding github releases
