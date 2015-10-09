use regex::Regex;

/// Splits DateString, TimeString
///
/// for further parsing by `parse_iso_8601_date` and `parse_iso_8601_time`.
pub fn split_iso_8601(string:&str) -> Option<(String, String)>
{
    let split = Regex::new(r"^([^T]*)T?(.*)$").unwrap();
    if split.is_match(&string) {
        let caps = split.captures(&string).unwrap();
        if caps.len() > 1 {
            return Some( (caps.at(1).unwrap().into(), caps.at(2).unwrap().into()) );
        }
    }
    None
}

/// Parses a ISO 8601 strin into LocalDateTime Object.
pub fn parse_iso_8601(string:&str) -> ((u32,u32,u32),(i8,i8,i8,i32))
{
    let (date_string, time_string) = split_iso_8601(string).unwrap();
    match (parse_iso_8601_date(&date_string), parse_iso_8601_time(&time_string)) {
        (Some((a,b,c)), Some((d,e,f,g))) => ((a,b,c),(d,e,f,g)),
        _ => panic!()
    }
}

/// Parses ISO 8601 Date strings into LocalDate Object.
pub fn parse_iso_8601_date(string:&str) -> Option<(u32, u32, u32)>
{
    let week = Regex::new(r##"(?x)^
        (\d{4})   # year
        -W(\d{2}) # number of week
        -(\d{1})  # day in week (1..7)
        $"##).unwrap();
    let ymd  = Regex::new(r##"(?x)^
        (\d{4})   # year
        -?(\d{2}) # month
        -?(\d{2}) # day
        $"##).unwrap();

    if ymd.is_match(&string) {
        ymd.captures(string).map(|caps|
        (
            caps.at(1).unwrap().parse().unwrap(), // year
            caps.at(2).unwrap().parse().unwrap(), // month
            caps.at(3).unwrap().parse().unwrap(), // day
            ))
    }
    else if week.is_match(&string) {
        week.captures(string).map(|caps|
        (
            caps.at(1).unwrap().parse().unwrap(), // year
            caps.at(2).unwrap().parse().unwrap(), // week
            caps.at(3).unwrap().parse().unwrap()  // weekday
            ))
    }
    else { None }
}

pub fn parse_iso_8601_time(string:&str) -> Option<(i8, i8, i8, i32)>
{
    if let Some((hour, minute, second, millisecond, _zh, _zm, _z)) = parse_iso_8601_tuple(string){
        return Some((hour, minute, second, millisecond));
    }
    None
}

fn parse_iso_8601_tuple(string:&str) -> Option<(i8,i8,i8,i32,i8,i8,&str)>
{
    let exp = Regex::new(r##"(?x) ^
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
    $"##).ok().expect("Regex Broken");

    if exp.is_match(&string) {
        let tup = exp.captures(string).map(|caps|
               (
                caps.at(1).unwrap_or("00").parse::<i8>().unwrap(), // HH
                caps.at(2).unwrap_or("00").parse::<i8>().unwrap(), // MM
                caps.at(3).unwrap_or("00").parse::<i8>().unwrap(), // SS
                caps.at(4).unwrap_or("000").parse::<i32>().unwrap(), // MS
                caps.at(6).unwrap_or("+00").trim_matches('+').parse::<i8>().unwrap(), // ZH
                caps.at(7).unwrap_or("00").parse::<i8>().unwrap(), // ZM
                caps.at(5).unwrap_or("_"), // "Z"
                )).unwrap();

        if tup.3 > 0 && &format!("{}", tup.3).len() %3 != 0{
            println!("{}", tup.3); return None}
        return Some(tup);

    }
    None
}

