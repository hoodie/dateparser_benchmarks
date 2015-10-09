#![feature(test)]
extern crate test;

extern crate chrono;
extern crate datetime;
extern crate iso8601;

static DATESTRING:&'static str = "2014-11-28T12:00:09Z";

#[cfg(test)]
mod chrono_bench{

    use super::test::Bencher;
    use chrono::*;

    #[bench]
    fn parse_iso8601(b: &mut Bencher) {
        b.iter(||{
            super::DATESTRING.parse::<DateTime<UTC>>()
        });
    }
}


#[cfg(test)]
mod datetime_bench{

    use super::test::Bencher;
    use datetime::local::*;

    #[bench]
    fn parse_iso8601(b: &mut Bencher) {
        b.iter(||{
            LocalDate::parse(super::DATESTRING);
        });
    }
}

#[cfg(test)]
mod nomdate_bench{

    use super::test::Bencher;
    use iso8601::easy::datetime;

    #[bench]
    fn parse_iso8601(b: &mut Bencher) {
        b.iter(||{
            datetime(super::DATESTRING.as_bytes());
        });
    }
}
