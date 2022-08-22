# covtests
Play repo to test out `cargo-tarpaulin` to examine a repo's test coverage, and to post results to
[Codecov](https://app.codecov.io/gh/rrybarczyk/covtests).

## Run Code Coverage Locally
Install `cargo-tarpaulin` globally:
```
cargo install cargo-tarpaulin
```

Run and print to `stdout`:
```
cargo +nightly tarpaulin --verbose --all-features --workspace --timeout 120 --out Stdout
```

Other print options include `Json`, `Xml`, `Html`, `Lcov`.
