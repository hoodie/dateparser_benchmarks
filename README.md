#Comparing Datetime libs in Rust

 This Benchmark Suite compares:
 
 * [chrono](https://crates.io/crates/chrono)
 * [datetime](https://crates.io/crates/datetime) (uses regex)
 * `datetime_regex_pure` (extracted from [datetime](https://crates.io/crates/datetime), but no validation)
 * [nom](https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/) (no validation yet, TODO)

even though the nom bench does not validate, the differences are already significant

## Benchmark of each implementation

```bash
$ cargo bench
     Running target/release/dateparser_benchmarks-71747040d4b5a03b

running 5 tests
test chrono_bench::parse_iso8601             ... bench:         873 ns/iter (+/- 68)
test datetime_bench::parse_iso8601           ... bench:      82,149 ns/iter (+/- 2,087)
test datetime_regex_pure_bench::apply_regex  ... bench:       2,292 ns/iter (+/- 54)
test datetime_regex_pure_bench::create_regex ... bench:      67,915 ns/iter (+/- 795)
test nomdate_bench::parse_iso8601            ... bench:         230 ns/iter (+/- 2)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured
```

## Completeness test of each implementation

```bash
$ cargo test -- --nocapture
     Running target/debug/dateparser_benchmarks-71747040d4b5a03b

running 6 tests
test chrono_bench::parse_iso8601 ... ok
test datetime_bench::parse_iso8601 ... ok
test datetime_regex_pure_bench::apply_regex ... ok
2015-10-24T16:30:48+00:00
 -> chrono:   Ok(2015-10-24T16:30:48Z)
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Done([], 2015-10-24T16:30:48Z0000)

2015-10-24T16:30:48Z
 -> chrono:   Ok(2015-10-24T16:30:48Z)
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Done([], 2015-10-24T16:30:48Z0000)

20151024T163048Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Error(Position(0, [49, 48, 50, 52, 84, 49, 54, 51, 48, 52, 56, 90]))

2015-W43T16:30:48Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Err(InvalidCharacter)
 -> nom:      Error(Position(6, [87, 52]))

2015-W43-6T16:30:48Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Error(Position(6, [87, 52]))

2015-297T16:30:48Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Err(InvalidCharacter)
 -> nom:      Error(Position(0, [55, 84, 49, 54, 58, 51, 48, 58, 52, 56, 90]))

test nomdate_bench::parse_iso8601 ... ok
test datetime_regex_pure_bench::create_regex ... ok
test completeness::iso_week_date ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests dateparser_benchmarks

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

```

