#Comparing Datetime libs in Rust

 This Benchmark Suite compares:
 
 * [chrono](https://crates.io/crates/chrono)
 * [datetime](https://crates.io/crates/datetime) (uses regex)
 * `datetime_regex_pure` (extracted from [datetime](https://crates.io/crates/datetime), but no validation)
 * [nom](https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/) (no validation yet, TODO)

even though the nom bench does not validate, the differences are already significant

```bash
     Running target/release/dateparser_benchmarks-71747040d4b5a03b

running 4 tests
test chrono_bench::parse_iso8601              ... bench:         819 ns/iter (+/- 7)
test datetime_bench::parse_iso8601            ... bench:      63,174 ns/iter (+/- 2,067)
test datetime_regex_pure_bench::parse_iso8601 ... bench:       1,700 ns/iter (+/- 71)
test nomdate_bench::parse_iso8601             ... bench:         265 ns/iter (+/- 121)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured
```

## TODO

* [ ] properly extract `DateTime` of the nom version

