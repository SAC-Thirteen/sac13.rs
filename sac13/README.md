# SAC13

SAC13 is a thirteen-month solar calendar in which every month has the same four-week layout (28 days*) and this library is the Rust reference implementation.

You can find details about SAC13 here: [https://sac13.net/](https://sac13.net/)

This library provides the data types and functions to convert from the Gregorian Calendar to SAC13 and vice versa. It also provides conversions from and to [Julian Day Numbers](https://en.wikipedia.org/wiki/Julian_day) if you wan't to interop with other calendar systems.

## Quick start
_TODO_

## Breaking Changes
Note, this library is pretty new so there will be lots breaking changes at least until we hit `0.1`. The plan is to keep it relatively stable after that with the help of semver-checks.

## Planned Features
- features for `time` and `chrono` interop
- nostd support
- wasm compatible