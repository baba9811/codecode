# Third-Party Licenses

This file summarizes third-party dependency license metadata for the locked
practicode dependency graph, checked from Cargo.lock and Cargo metadata.

This is not legal advice.

## Project License

practicode is licensed under MIT. The current dependency set is compatible with
that choice: all Rust dependencies use permissive licenses, and the npm package
has no npm dependencies.

## npm Dependencies

None.

## Direct Rust Dependencies

| Crate | Version | License |
| --- | --- | --- |
| anyhow | 1.0.103 | MIT OR Apache-2.0 |
| crossterm | 0.29.0 | MIT |
| ratatui | 0.30.2 | MIT |
| serde | 1.0.228 | MIT OR Apache-2.0 |
| serde_json | 1.0.150 | MIT OR Apache-2.0 |
| unicode-width | 0.2.0 | MIT OR Apache-2.0 |
| wait-timeout | 0.2.1 | MIT/Apache-2.0 |

## Transitive Rust Dependency License Groups

The locked Rust dependency graph contains 94 third-party packages.

| License expression | Packages |
| --- | ---: |
| MIT OR Apache-2.0 | 53 |
| MIT | 23 |
| MIT/Apache-2.0 | 5 |
| Apache-2.0 OR MIT | 3 |
| Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | 3 |
| (MIT OR Apache-2.0) AND Unicode-3.0 | 1 |
| Apache-2.0 | 1 |
| Apache-2.0 OR BSL-1.0 | 1 |
| Apache-2.0/MIT | 1 |
| MIT OR Apache-2.0 OR CC0-1.0 | 1 |
| Unlicense OR MIT | 1 |
| Zlib | 1 |

No GPL, LGPL, AGPL, or MPL dependencies were detected in the locked Rust
dependency graph.

## Release Notes

The crates.io and npm packages distribute practicode source, not vendored
third-party source or prebuilt third-party binaries. The npm installer builds
the Rust binary locally with Cargo.

If practicode later ships prebuilt binaries, include full third-party notices
and license texts with those binary artifacts.
