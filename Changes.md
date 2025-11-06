# Changelog

## [0.7] (2024-11-06)

- Added new cli option ``-i`` / ``--stdin`` to read commands from a pipe
- When in piping mode, output is not pretty printed (pipe to ``jq`` for more fun)
- Split main.rs up since it was getting kinda big
- Messages from the api.rs module use the eprintln! macro so that piping output doesn't get muddled


## [0.6] (2024-10-30)

- Added command history support via readline

## [0.5] (2024-10-22)

- Reduce the number of keypresses required to end a multiline command
- Add support for terminating commands with the ; character

## [0.4] (2024-10-21)

- Fix github build action (was failing for aarm64)

## [0.3] (2025-10-21)

- Fix regular queries (previously only dispatched EXPLAIN queries)

## [0.2] (2025-10-20)

- Fix github actions for build/release

## [0.1] (2025-10-19)

- Initial REPL release
- Adding github releases
