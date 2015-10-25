#![feature(test)]
extern crate test;

extern crate chrono;
extern crate datetime;
extern crate iso8601;
extern crate nom;
extern crate regex;

mod datetime_regex_pure;

static DATESTRING: &'static str = "2014-11-28T12:00:09Z";

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
mod datetime_regex_pure_bench{

    use super::test::Bencher;
    use datetime_regex_pure::PureRegexParser;

    #[bench]
    fn create_regex(b: &mut Bencher) {
        b.iter(||{
            PureRegexParser::new();
        });
    }

    #[bench]
    fn apply_regex(b: &mut Bencher) {
        let parser = PureRegexParser::new();
        b.iter(||{
            parser.parse_iso_8601(super::DATESTRING);
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
            LocalDateTime::parse(super::DATESTRING);
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

