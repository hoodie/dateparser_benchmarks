# Comparing Datetime libs in Rust

 This Benchmark Suite compares:

 * [chrono 0.2](https://crates.io/crates/chrono)
 * [iso8601 0.1](https://crates.io/crates/iso8601) (pure parsing, no validation, see [blog post](https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/))
 * [datetime](https://crates.io/crates/datetime) (used to use regex, just uses iso8601 today)
 * [dtparse 1.0](https://crates.io/crates/dtparse)



## Tests

```bash
running 14 tests
test chrono_bench::parse_iso8601 ... ok
test completeness::all_nom ... ok
test completeness::minimal_chrono ... ok
test completeness::all_chrono ... FAILED
test completeness::all_datetime ... ok
test completeness::minimal_datetime ... ok
test completeness::minimal_dtparse ... ok
test completeness::minimal_nom ... ok
test iso8601_bench::parse_iso8601 ... ok
test completeness::all_dtparse ... FAILED
test datetime_bench::parse_iso8601 ... ok
test dtparse_bench::parse_dtparse ... ok
test completeness::iso_week_date ... ok
test regex_bench::parse_iso8601 ... ok

failures:

---- completeness::all_chrono stdout ----
thread 'completeness::all_chrono' panicked at 'called `Result::unwrap()` on an `Err` value: ParseError(Invalid)', src/libcore/result.rs:999:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

---- completeness::all_dtparse stdout ----
thread 'completeness::all_dtparse' panicked at 'called `Result::unwrap()` on an `Err` value: YearMonthDayError("Year already set")', src/libcore/result.rs:999:5


failures:
    completeness::all_chrono
    completeness::all_dtparse

test result: FAILED. 12 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

## Benches

```bash
running 14 tests
test completeness::all_chrono ... ignored
test completeness::all_datetime ... ignored
test completeness::all_dtparse ... ignored
test completeness::all_nom ... ignored
test completeness::iso_week_date ... ignored
test completeness::minimal_chrono ... ignored
test completeness::minimal_datetime ... ignored
test completeness::minimal_dtparse ... ignored
test completeness::minimal_nom ... ignored
test chrono_bench::parse_iso8601   ... bench:         253 ns/iter (+/- 4)
test datetime_bench::parse_iso8601 ... bench:         214 ns/iter (+/- 11)
test dtparse_bench::parse_dtparse  ... bench:       2,683 ns/iter (+/- 107)
test iso8601_bench::parse_iso8601  ... bench:         174 ns/iter (+/- 6)
test regex_bench::parse_iso8601    ... bench:         649 ns/iter (+/- 27)

test result: ok. 0 passed; 0 failed; 9 ignored; 5 measured; 0 filtered out
```