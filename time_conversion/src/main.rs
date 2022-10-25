use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

enum AmPm {
    Am,
    Pm
}

impl From<&str> for AmPm {
    fn from(s: &str) -> Self {
        if s.eq_ignore_ascii_case("pm") {
            Self::Pm
        } else {
            Self::Am
        }
    }
}

struct TimeTwelveHr {
    hrs: u8,
    mins: u8,
    secs: u8,
    am_pm: AmPm,
}

impl From<&str> for TimeTwelveHr {
    fn from(s: &str) -> Self {
        let (time, am_pm) = s.split_at(s.len()-2);
        let time: Vec<&str> = time.split(":").collect();
        TimeTwelveHr {
            hrs: u8::from_str(time[0]).unwrap(),
            mins: u8::from_str(time[1]).unwrap(),
            secs: u8::from_str(time[2]).unwrap(),
            am_pm: AmPm::from(am_pm)
        }
    }
}

struct TimeTwentyFourHr {
    hrs: u8,
    mins: u8,
    secs: u8,
}

impl From<&TimeTwelveHr> for TimeTwentyFourHr {
    fn from(t: &TimeTwelveHr) -> Self {
        let hrs = match t.am_pm {
            AmPm::Am => t.hrs % 12,
            AmPm::Pm => t.hrs % 12 + 12
        };

        TimeTwentyFourHr {
            hrs,
            mins: t.mins,
            secs: t.secs
        }
    }
}

impl ToString for TimeTwentyFourHr {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hrs, self.mins, self.secs)
    }
}


fn timeConversion(s: &str) -> String {
    TimeTwentyFourHr::from(&TimeTwelveHr::from(s)).to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
