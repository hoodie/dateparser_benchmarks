# Comparing Datetime libs in Rust

 This Benchmark Suite compares:

 * [chrono](https://crates.io/crates/chrono)
 * [iso8601](https://crates.io/crates/iso8601) (pure parsing, no validation, see [blog post](https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/))
 * [datetime](https://crates.io/crates/datetime) (uses iso8601 0.1)
 * [dtparse](https://crates.io/crates/dtparse)

## Benches 

```bash
test chrono02_bench::parse_iso8601       ... bench:         229 ns/iter (+/- 31)
test chrono03_bench::parse_iso8601       ... bench:         419 ns/iter (+/- 39)
test chrono04_bench::parse_iso8601       ... bench:         426 ns/iter (+/- 37)
test datetime_bench::parse_iso8601       ... bench:         166 ns/iter (+/- 37)
test dtparse_bench::parse_iso8601        ... bench:       7,108 ns/iter (+/- 1,597)
test iso8601_v01_bench::parse_iso8601    ... bench:         130 ns/iter (+/- 22)
test iso8601_v03_bench::parse_iso8601    ... bench:         145 ns/iter (+/- 22)
test iso8601_v04_bench::parse_iso8601    ... bench:         119 ns/iter (+/- 19)
test iso8601_master_bench::parse_iso8601 ... bench:         142 ns/iter (+/- 31) // oops
test regex_bench::parse_iso8601          ... bench:         616 ns/iter (+/- 147)
```
