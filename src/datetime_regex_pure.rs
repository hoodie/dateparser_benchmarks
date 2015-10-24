use regex::Regex;

pub struct PureRegexParser {
    split: Regex,
    week: Regex,
    ymd: Regex,
    exp: Regex,
}

impl PureRegexParser {
    pub fn new() -> Self {
        PureRegexParser {
            split: Regex::new(r"^([^T]*)T?(.*)$").unwrap(),
            week: Regex::new(r##"(?x)^
                (\d{4})   # year
                -W(\d{2}) # number of week
                -(\d{1})  # day in week (1..7)
                $"##).unwrap(),
            ymd: Regex::new(r##"(?x)^
                (\d{4})   # year
                -?(\d{2}) # month
                -?(\d{2}) # day
                $"##).unwrap(),
            exp: Regex::new(r##"(?x) ^
                (\d{2}) :?     # hour
                (\d{2})? :?    # minute

                (?:
                    (\d{2})         # second
                    \.?
                    ((?:\d{1,9}))?  # millisecond
                )?

                (?:                 # time zone offset:
                    (Z) |           # or just Z for UTC
                    ([+-]\d\d)? :?  # hour and
                    (\d\d)?         # minute,
                )?
            $"##).expect("Regex broken"),
        }
    }

    /// Splits DateString, TimeString
    ///
    /// for further parsing by `parse_iso_8601_date` and `parse_iso_8601_time`.
    pub fn split_iso_8601(&self, string: &str) -> Option<(String, String)>
    {
        if let Some(caps) = self.split.captures(&string) {
            if caps.len() > 1 {
                return Some((caps.at(1).unwrap().into(), caps.at(2).unwrap().into()));
            }
        }
        None
    }

    /// Parses a ISO 8601 strin into LocalDateTime Object.
    pub fn parse_iso_8601(&self, string: &str) -> ((u32,u32,u32),(i8,i8,i8,i32))
    {
        let (date_string, time_string) = self.split_iso_8601(string).unwrap();
        match (self.parse_iso_8601_date(&date_string), self.parse_iso_8601_time(&time_string)) {
            (Some((a,b,c)), Some((d,e,f,g))) => ((a,b,c),(d,e,f,g)),
            _ => panic!()
        }
    }

    /// Parses ISO 8601 Date strings into LocalDate Object.
    pub fn parse_iso_8601_date(&self, string: &str) -> Option<(u32, u32, u32)>
    {
        if let Some(caps) = self.ymd.captures(&string) {
            Some((caps.at(1).unwrap().parse().unwrap(), // year
                  caps.at(2).unwrap().parse().unwrap(), // month
                  caps.at(3).unwrap().parse().unwrap(), // day
                ))
        }
        else if let Some(caps) = self.week.captures(&string) {
            Some((caps.at(1).unwrap().parse().unwrap(), // year
                 caps.at(2).unwrap().parse().unwrap(), // week
                 caps.at(3).unwrap().parse().unwrap()  // weekday
                ))
        }
        else { None }
    }

    pub fn parse_iso_8601_time(&self, string: &str) -> Option<(i8, i8, i8, i32)>
    {
        if let Some((hour, minute, second, millisecond, _zh, _zm, _z)) = self.parse_iso_8601_tuple(string) {
            return Some((hour, minute, second, millisecond));
        }
        None
    }

    fn parse_iso_8601_tuple<'a>(&self, string: &'a str) -> Option<(i8,i8,i8,i32,i8,i8,&'a str)>
    {
        if let Some(caps) = self.exp.captures(&string) {
            let tup = (caps.at(1).unwrap_or("00").parse::<i8>().unwrap(), // HH
                       caps.at(2).unwrap_or("00").parse::<i8>().unwrap(), // MM
                       caps.at(3).unwrap_or("00").parse::<i8>().unwrap(), // SS
                       caps.at(4).unwrap_or("000").parse::<i32>().unwrap(), // MS
                       caps.at(6).unwrap_or("+00").trim_matches('+').parse::<i8>().unwrap(), // ZH
                       caps.at(7).unwrap_or("00").parse::<i8>().unwrap(), // ZM
                       caps.at(5).unwrap_or("_"), // "Z"
                      );
            if tup.3 > 0 && caps.at(4).map(|ms| ms.len()).unwrap_or(0) % 3 != 0 {
                // println!("{}", tup.3);
                return None
            }
            return Some(tup);
        }
        None
    }
}
