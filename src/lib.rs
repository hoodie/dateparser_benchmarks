#![feature(test)]
#![allow(unused_must_use)]
#![allow(dead_code)]
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
    use std::str::FromStr;
    use datetime::local::*;

    #[bench]
    fn parse_iso8601(b: &mut Bencher) {
        b.iter(||{
            LocalDateTime::from_str(super::DATESTRING);
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


#[cfg(test)]
mod completeness{

    use chrono::*;
    use std::str::FromStr;
    use datetime::local::LocalDateTime;
    use iso8601::easy::datetime as nomdatetime;

    #[test]
    fn iso_week_date() {

        let tests = [
            "2015-10-24T16:30:48+00:00",
            "2015-10-24T16:30:48Z",
            "20151024T163048Z",
            "2015-W43T16:30:48Z",
            "2015-W43-6T16:30:48Z",
            "2015-297T16:30:48Z",
        ];

        for date in tests.iter(){
            let parsed_chrono = date.parse::<DateTime<UTC>>();
            let parsed_datetime = LocalDateTime::from_str(date);
            let parsed_nom = nomdatetime(date.as_bytes());
            println!("{}\n -> chrono:   {:?}\n -> datetime: {:?}\n -> nom:      {:?}\n", date, parsed_chrono, parsed_datetime, parsed_nom);
        }
    }

}
