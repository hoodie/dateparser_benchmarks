#Comparing Datetime libs in Rust

 This Benchmark Suite compares:
 
 * [chrono](https://crates.io/crates/chrono)
 * [datetime](https://crates.io/crates/datetime) (used to use regex)
 * [nom](https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/) (pure parsing, no validation yet)


## Benchmark of each implementation

```bash
     Running target/release/dateparser_benchmarks-71747040d4b5a03b

running 4 tests
test completeness::iso_week_date ... ignored
test chrono_bench::parse_iso8601   ... bench:         676 ns/iter (+/- 693)
test datetime_bench::parse_iso8601 ... bench:         457 ns/iter (+/- 6)
test nomdate_bench::parse_iso8601  ... bench:         386 ns/iter (+/- 6)
```

Check out [this commit](https://github.com/hoodie/dateparser_benchmarks/tree/2aa390ad2b8750c9dd606100e66b51b461d22b0d) to see what used to be the problem here


## Completeness test of each implementation

```bash
     Running target/debug/dateparser_benchmarks-71747040d4b5a03b

running 4 tests
2015-10-24T16:30:48+00:00
 -> chrono:   Ok(2015-10-24T16:30:48Z)
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Ok(DateTime { date: YMD { year: 2015, month: 10, day: 24 }, time: Time { hour: 16, minute: 30, second: 48, millisecond: 0, tz_offset_hours: 0, tz_offset_minutes: 0 } })

2015-10-24T16:30:48Z
 -> chrono:   Ok(2015-10-24T16:30:48Z)
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Ok(DateTime { date: YMD { year: 2015, month: 10, day: 24 }, time: Time { hour: 16, minute: 30, second: 48, millisecond: 0, tz_offset_hours: 0, tz_offset_minutes: 0 } })

test chrono_bench::parse_iso8601 ... ok
test datetime_bench::parse_iso8601 ... ok
20151024T163048Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Ok(DateTime { date: YMD { year: 2015, month: 10, day: 24 }, time: Time { hour: 16, minute: 30, second: 48, millisecond: 0, tz_offset_hours: 0, tz_offset_minutes: 0 } })

test nomdate_bench::parse_iso8601 ... ok
2015-W436T16:30:48Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Ok(DateTime { date: Week { year: 2015, ww: 43, d: 6 }, time: Time { hour: 16, minute: 30, second: 48, millisecond: 0, tz_offset_hours: 0, tz_offset_minutes: 0 } })

2015-W43-6T16:30:48Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Ok(DateTime { date: Week { year: 2015, ww: 43, d: 6 }, time: Time { hour: 16, minute: 30, second: 48, millisecond: 0, tz_offset_hours: 0, tz_offset_minutes: 0 } })

2015-297T16:30:48Z
 -> chrono:   Err(ParseError(Invalid))
 -> datetime: Ok(LocalDateTime { date: LocalDate { ymd: YMD { year: 2015, month: October, day: 24 }, yearday: 297, weekday: Saturday }, time: LocalTime { hour: 16, minute: 30, second: 48, millisecond: 0 } })
 -> nom:      Ok(DateTime { date: Ordinal { year: 2015, ddd: 297 }, time: Time { hour: 16, minute: 30, second: 48, millisecond: 0, tz_offset_hours: 0, tz_offset_minutes: 0 } })

test completeness::iso_week_date ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests dateparser_benchmarks

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```
