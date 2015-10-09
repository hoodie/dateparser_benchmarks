#Comparing Datetime libs in Rust

 This Benchmark Suite compares:
 
 * `chrono`
 * `datetime`
 * `datetime_regex_pure` (extracted, no validation)
 * [nom](https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/) (no validation, TODO)

```bash
     Running target/release/dateparser_benchmarks-71747040d4b5a03b

running 4 tests
test chrono_bench::parse_iso8601              ... bench:         866 ns/iter (+/- 3)
test datetime_bench::parse_iso8601            ... bench:      82,033 ns/iter (+/- 847)
test datetime_regex_pure_bench::parse_iso8601 ... bench:      82,847 ns/iter (+/- 2,394)
test nomdate_bench::parse_iso8601             ... bench:         260 ns/iter (+/- 2)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured
```

## TODO

* [ ] properly extract `DateTime` of the nom version
