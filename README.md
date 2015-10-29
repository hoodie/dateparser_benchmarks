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

running 6 tests
test completeness::iso_week_date ... ignored
test chrono_bench::parse_iso8601             ... bench:         703 ns/iter (+/- 10)
test datetime_bench::parse_iso8601           ... bench:      52,129 ns/iter (+/- 3,308)
test datetime_regex_pure_bench::apply_regex  ... bench:       1,484 ns/iter (+/- 137)
test datetime_regex_pure_bench::create_regex ... bench:      41,473 ns/iter (+/- 1,217)
test nomdate_bench::parse_iso8601            ... bench:         403 ns/iter (+/- 17)
test result: ok. 0 passed; 0 failed; 1 ignored; 5 measured
```

Check out [this branch](https://github.com/hoodie/dateparser_benchmarks/tree/nightly_datetime#benchmark-of-each-implementation) to see what's possible

## Completeness test of each implementation

```bash
$ cargo test -- --nocapture
     Running target/debug/dateparser_benchmarks-71747040d4b5a03b

running 6 tests
test datetime_bench::parse_iso8601 ... ok
test chrono_bench::parse_iso8601 ... ok
test datetime_regex_pure_bench::apply_regex ... ok
2015-10-24T16:30:48+00:00
 -> chrono:   Ok(2015-10-24T16:30:48Z)
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Done([], DateTime { date: YMD { year: 2015, month: 10, day: 24 }, time: Time { hour: 16, minute: 30, second: 48, tz_offset: 0 } })

2015-10-24T16:30:48Z
 -> chrono:   Ok(2015-10-24T16:30:48Z)
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Done([], DateTime { date: YMD { year: 2015, month: 10, day: 24 }, time: Time { hour: 16, minute: 30, second: 48, tz_offset: 0 } })

20151024T163048Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Done([52, 56, 90], DateTime { date: YMD { year: 2015, month: 10, day: 24 }, time: Time { hour: 16, minute: 30, second: 0, tz_offset: 0 } })

2015-W436T16:30:48Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Err(InvalidCharacter)
 -> nom:      Done([], DateTime { date: Week { year: 2015, ww: 43, d: 6 }, time: Time { hour: 16, minute: 30, second: 48, tz_offset: 0 } })

2015-W43-6T16:30:48Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Done([], DateTime { date: Week { year: 2015, ww: 43, d: 6 }, time: Time { hour: 16, minute: 30, second: 48, tz_offset: 0 } })

2015-297T16:30:48Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Err(InvalidCharacter)
 -> nom:      Done([], DateTime { date: Ordinal { year: 2015, ddd: 297 }, time: Time { hour: 16, minute: 30, second: 48, tz_offset: 0 } })

test completeness::iso_week_date ... ok
test nomdate_bench::parse_iso8601 ... ok
test datetime_regex_pure_bench::create_regex ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests dateparser_benchmarks

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```


