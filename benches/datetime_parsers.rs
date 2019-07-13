

static DATESTRING: &'static str = "2014-11-28T12:00:09Z";

use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn parse_with_chrono_v02(c: &mut Criterion) {
    use chrono_v02::*;
    c.bench_function("chrono 0.2", |b| b.iter(|| black_box(DATESTRING.parse::<DateTime<UTC>>())));
}

fn parse_with_chrono_v03(c: &mut Criterion) {
    use chrono_v03::*;
    c.bench_function("chrono 0.3", |b| b.iter(|| black_box(DATESTRING.parse::<DateTime<UTC>>())));
}

fn parse_with_chrono_v04(c: &mut Criterion) {
    use chrono_v04::*;
    c.bench_function("chrono 0.4", |b| b.iter(|| black_box(DATESTRING.parse::<DateTime<Utc>>())));
}

fn parse_with_naive_regex(c: &mut Criterion) {
    use datetimeparser_benchmarks::datetime_regex_pure::PureRegexParser;
    c.bench_function("regex", |b| {
        let parser = PureRegexParser::new();
        b.iter(|| black_box(parser.parse_iso_8601(DATESTRING)))
    });
}

 fn parse_with_naive_datetime(c: &mut Criterion) {
    use datetime::*;
    use std::str::FromStr;
    c.bench_function("datetime", |b| b.iter(|| black_box(
        LocalDateTime::from_str(DATESTRING)
    )));
}

 fn parse_with_iso8601_v1(c: &mut Criterion) {
    use iso8601_v01::datetime;
    c.bench_function("iso8601 0.1", |b| b.iter(|| black_box(
        datetime(DATESTRING)
    )));
}

 fn parse_with_iso8601_v3(c: &mut Criterion) {
    use iso8601_v03::datetime;
    c.bench_function("iso8601 0.3", |b| b.iter(|| black_box(
        datetime(DATESTRING)
    )));
}

 fn parse_with_dtparse(c: &mut Criterion) {
    c.bench_function("dtparse", |b| b.iter(|| black_box(
        dtparse::parse(DATESTRING)
    )));
}

criterion_group!(benches, parse_with_chrono_v02,
parse_with_chrono_v03,
parse_with_chrono_v04,
parse_with_naive_regex,
parse_with_iso8601_v1,
parse_with_iso8601_v3,
parse_with_dtparse);
criterion_main!(benches);