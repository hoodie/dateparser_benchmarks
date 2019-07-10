#![feature(test)]
#![allow(unused_must_use)]
#![allow(dead_code)]
extern crate test;

extern crate chrono02;
extern crate chrono03;
extern crate chrono04;
extern crate iso8601_v01;
extern crate iso8601_v03;
extern crate datetime;
extern crate dtparse;
extern crate regex;

mod datetime_regex_pure;

static DATESTRING: &'static str = "2014-11-28T12:00:09Z";

#[cfg(test)]
mod chrono02_bench {

    use super::test::Bencher;
    use chrono02::*;

    #[bench]
    fn parse_iso8601(b: &mut Bencher) {
        b.iter(||{
            super::DATESTRING.parse::<DateTime<UTC>>()
        });
    }
}

#[cfg(test)]
mod chrono03_bench {

    use super::test::Bencher;
    use chrono03::*;

    #[bench]
    fn parse_iso8601(b: &mut Bencher) {
        b.iter(||{
            super::DATESTRING.parse::<DateTime<UTC>>()
        });
    }
}

#[cfg(test)]
mod chrono04_bench {

    use super::test::Bencher;
    use chrono04::*;

    #[bench]
    fn parse_iso8601(b: &mut Bencher) {
        b.iter(||{
            super::DATESTRING.parse::<DateTime<Utc>>()
        });
    }
}

#[cfg(test)]
mod regex_bench{

    use super::test::Bencher;
    use datetime_regex_pure::PureRegexParser;

    //#[bench]
    fn create_regex(b: &mut Bencher) {
        b.iter(||{
            PureRegexParser::new();
        });
    }

    #[bench]
    fn parse_iso8601(b: &mut Bencher) {
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
    use datetime::*;

    #[bench]
    fn parse_iso8601(b: &mut Bencher) {
        b.iter(||{
            LocalDateTime::from_str(super::DATESTRING);
        });
    }
}

#[cfg(test)]
mod iso8601_v01_bench{

    use super::test::Bencher;
    use iso8601_v01::datetime;

    #[bench]
    fn parse_iso8601(b: &mut Bencher) {
        b.iter(||{
            datetime(super::DATESTRING);
        });
    }
}


#[cfg(test)]
mod iso8601_v03_bench{

    use super::test::Bencher;
    use iso8601_v03::datetime;

    #[bench]
    fn parse_iso8601(b: &mut Bencher) {
        b.iter(||{
            datetime(super::DATESTRING);
        });
    }
}


#[cfg(test)]
mod dtparse_bench{

    use super::test::Bencher;

    #[bench]
    fn parse_dtparse(b: &mut Bencher) {
        b.iter(||{
            dtparse::parse(super::DATESTRING);
        });
    }
}


#[cfg(test)]
mod completeness{

    use std::str::FromStr;
    use datetime::LocalDateTime;
    use iso8601_v03::datetime as nomdatetime;

    static ALL_FORMATS: &'static [&'static str] = &[
        "2015-10-24T16:30:48+00:00",
        "2015-10-24T16:30:48Z",
        "20151024T163048Z",
        "2015-W436T16:30:48Z",
        "2015-W43-6T16:30:48Z",
        "2015-297T16:30:48Z",
    ];


    static MINIMAL_FORMATS: &'static [&'static str] = &[
        "2015-10-24T16:30:48+00:00",
        "2015-10-24T16:30:48Z",
    ];


    #[test]
    fn iso_week_date() {
        for date in ALL_FORMATS.iter(){
            let parsed_by_chrono02 = date.parse::<chrono02::DateTime<chrono02::UTC>>();
            let parsed_by_datetime = LocalDateTime::from_str(date);
            let parsed_by_nom = nomdatetime(date);
            let parsed_by_dtparse = dtparse::parse(date);
            println!("{}\n -> chrono:   {:?}\n -> datetime: {:?}\n -> nom:      {:?}\n -> dtparse:  {:?}\n",
            date,
            parsed_by_chrono02,
            parsed_by_datetime,
            parsed_by_nom,
            parsed_by_dtparse
            );
        }
    }

    #[test]
    fn minimal_chrono02() {
        use chrono02::{DateTime, UTC};
        for date in MINIMAL_FORMATS.iter() { date.parse::<DateTime<UTC>>().unwrap(); }
    }

    #[test]
    fn minimal_chrono04() {
        use chrono04::{DateTime, Utc};
        for date in MINIMAL_FORMATS.iter() { date.parse::<DateTime<Utc>>().unwrap(); }
    }

    #[test]
    fn minimal_datetime() { for date in MINIMAL_FORMATS.iter() { LocalDateTime::from_str(date).unwrap(); } }

    #[test]
    fn minimal_nom() { for date in MINIMAL_FORMATS.iter() { nomdatetime(date).unwrap(); } }

    #[test]
    fn minimal_dtparse() { for date in MINIMAL_FORMATS.iter() { dtparse::parse(date).unwrap(); } }


    #[test]
    fn all_chrono02() {
        use chrono02::{DateTime, UTC};
        for date in ALL_FORMATS.iter() { date.parse::<DateTime<UTC>>().unwrap(); }
    }

    #[test]
    fn all_chrono04() {
        use chrono04::{DateTime, Utc};
        for date in ALL_FORMATS.iter() { date.parse::<DateTime<Utc>>().unwrap(); }
    }

    #[test]
    fn all_datetime() { for date in ALL_FORMATS.iter() { LocalDateTime::from_str(date).unwrap(); } }

    #[test]
    fn all_nom() { for date in ALL_FORMATS.iter() { nomdatetime(date).unwrap(); } }

    #[test]
    fn all_dtparse() { for date in ALL_FORMATS.iter() { dtparse::parse(date).unwrap(); } }

}
