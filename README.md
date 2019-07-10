# Comparing Datetime libs in Rust

 This Benchmark Suite compares:

 * [chrono](https://crates.io/crates/chrono)
 * [iso8601](https://crates.io/crates/iso8601) (pure parsing, no validation, see [blog post](https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/))
 * [datetime](https://crates.io/crates/datetime) (used to use regex, just uses iso8601 today)
 * [dtparse](https://crates.io/crates/dtparse)

## Tests

```bash
test completeness::all_chrono02 ... FAILED
test completeness::all_chrono04 ... FAILED
test completeness::all_datetime ... ok
test completeness::all_dtparse ... FAILED
test completeness::all_nom ... ok
test completeness::iso_week_date ... ok
test completeness::minimal_chrono02 ... ok
test completeness::minimal_chrono04 ... ok
test completeness::minimal_datetime ... ok
test completeness::minimal_dtparse ... ok
test completeness::minimal_nom ... ok

failures:

---- completeness::all_chrono02 stdout ----
thread 'completeness::all_chrono02' panicked at 'called `Result::unwrap()` on an `Err` value: ParseError(Invalid)', src/libcore/result.rs:999:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

---- completeness::all_chrono04 stdout ----
thread 'completeness::all_chrono04' panicked at 'called `Result::unwrap()` on an `Err` value: ParseError(Invalid)', src/libcore/result.rs:999:5

---- completeness::all_dtparse stdout ----
thread 'completeness::all_dtparse' panicked at 'called `Result::unwrap()` on an `Err` value: YearMonthDayError("Year already set")', src/libcore/result.rs:999:5


failures:
    completeness::all_chrono02
    completeness::all_chrono04
    completeness::all_dtparse

test result: FAILED. 16 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
```

## Benches 

```bash
test chrono02_bench::parse_iso8601    ... bench:         288 ns/iter (+/- 11)
test chrono03_bench::parse_iso8601    ... bench:         613 ns/iter (+/- 31)
test chrono04_bench::parse_iso8601    ... bench:         632 ns/iter (+/- 55)
test datetime_bench::parse_iso8601    ... bench:         241 ns/iter (+/- 11)
test dtparse_bench::parse_iso8601     ... bench:       3,057 ns/iter (+/- 276)
test iso8601_v01_bench::parse_iso8601 ... bench:         211 ns/iter (+/- 9)
test iso8601_v03_bench::parse_iso8601 ... bench:         183 ns/iter (+/- 34)
test regex_bench::parse_iso8601       ... bench:         538 ns/iter (+/- 130)

test result: ok. 0 passed; 0 failed; 11 ignored; 8 measured; 0 filtered out

