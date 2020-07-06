extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(?x)
        (?P<year>\d{4})     # the year
        -
        (?P<month>\d{2})    # the month
        -
        (?P<day>\d{2})      # the day
        ").unwrap();
    let caps = re.captures("2018-01-01").unwrap();
    assert_eq!("2018", &caps["year"]);
    assert_eq!("01", &caps["month"]);
    assert_eq!("01", &caps["day"]);
    let after = re.replace_all("2018-01-01", "$month/$day/$year");
    assert_eq!(after, "01/01/2018");
}
