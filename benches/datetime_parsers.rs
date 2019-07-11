

use datetimeparser_benchmarks::datetime_regex_pure;

static DATESTRING: &'static str = "2014-11-28T12:00:09Z";

use criterion::{criterion_group, criterion_main, Criterion};
use chrono02::*;

fn parse_iso8601(c: &mut Criterion) {
    c.bench_function("chrono 0.2", |b| b.iter(|| DATESTRING.parse::<DateTime<UTC>>()));
}

criterion_group!(benches, parse_iso8601);
criterion_main!(benches);