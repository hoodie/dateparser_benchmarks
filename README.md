#Comparing Datetime libs in Rust

 This Benchmark Suite compares:
 
 * [chrono](https://crates.io/crates/chrono)
 * [datetime](https://crates.io/crates/datetime) (uses regex)
 * `datetime_regex_pure` (extracted from [datetime](https://crates.io/crates/datetime), but no validation)
 * [nom](https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/) (no validation yet, TODO)

even though the nom bench does not validate, the differences are already significant

```bash
     Running target/release/dateparser_benchmarks-71747040d4b5a03b

running 5 tests
test chrono_bench::parse_iso8601             ... bench:         873 ns/iter (+/- 68)
test datetime_bench::parse_iso8601           ... bench:      82,149 ns/iter (+/- 2,087)
test datetime_regex_pure_bench::apply_regex  ... bench:       2,292 ns/iter (+/- 54)
test datetime_regex_pure_bench::create_regex ... bench:      67,915 ns/iter (+/- 795)
test nomdate_bench::parse_iso8601            ... bench:         230 ns/iter (+/- 2)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured
```

## TODO

* [ ] properly extract `DateTime` of the nom version

