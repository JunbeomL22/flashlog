use once_cell::sync::Lazy;
use quanta::Clock;
use std::time::{SystemTime, UNIX_EPOCH};

const UNIX_NANO_ANCHOR_BUFFER: u64 = 10; //10ns

pub static UNIVERSIAL_CLOCK: Lazy<Clock> = Lazy::new(Clock::new);

#[inline]
pub fn get_unix_nano() -> u64 {
    static UNIVERSIAL_SYSTEMTIME_ANCHOR: Lazy<u64> = Lazy::new(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
            + UNIX_NANO_ANCHOR_BUFFER
    });
    static UNIVERSIAL_CLOCK_ANCHOR: Lazy<u64> = Lazy::new(|| UNIVERSIAL_CLOCK.raw());

    UNIVERSIAL_CLOCK.delta_as_nanos(*UNIVERSIAL_CLOCK_ANCHOR, UNIVERSIAL_CLOCK.raw())
        + *UNIVERSIAL_SYSTEMTIME_ANCHOR
}

pub fn time_components_from_unix_nano(unix_nano: u64) -> (u8, u8, u8, u16) {
    let total_seconds = unix_nano / 1_000_000_000;
    let nanos = unix_nano % 1_000_000_000;
    
    let seconds_of_day = total_seconds % 86400;
    
    let hours = (seconds_of_day / 3600) as u8;
    let minutes = ((seconds_of_day % 3600) / 60) as u8;
    let seconds = (seconds_of_day % 60) as u8;
    let millis = (nanos / 1_000_000) as u16;
    
    (hours, minutes, seconds, millis)
}

pub fn convert_unix_nano_to_date_and_time(unix_nano: u64, utc_offset_hour: i32) -> (String, String) {
    const NANOS_IN_SEC: u64 = 1_000_000_000;
    const NANOS_IN_MIN: u64 = 60 * NANOS_IN_SEC;
    const NANOS_IN_HOUR: u64 = 60 * NANOS_IN_MIN;
    const NANOS_IN_DAY: u64 = 24 * NANOS_IN_HOUR;

    let days_since_epoch = unix_nano / NANOS_IN_DAY;
    let remaining_nanos = unix_nano % NANOS_IN_DAY;

    let hours = remaining_nanos / NANOS_IN_HOUR;
    let remaining_nanos = remaining_nanos % NANOS_IN_HOUR;

    let minutes = remaining_nanos / NANOS_IN_MIN;
    let remaining_nanos = remaining_nanos % NANOS_IN_MIN;

    let seconds = remaining_nanos / NANOS_IN_SEC;
    let remaining_nanos = remaining_nanos % NANOS_IN_SEC;

    let millis = remaining_nanos / 1_000_000;
    let remaining_nanos = remaining_nanos % 1_000_000;

    let micros = remaining_nanos / 1_000;
    let nanos = remaining_nanos % 1_000;

    // Adjust for UTC offset
    let mut total_hours = hours as i32 + utc_offset_hour;
    let mut total_days = days_since_epoch as i32;

    if total_hours >= 24 {
        total_hours -= 24;
        total_days += 1;
    } else if total_hours < 0 {
        total_hours += 24;
        total_days -= 1;
    }

    let (year, month, day) = days_to_date(total_days as u32);

    let date = format!("{:04}{:02}{:02}", year, month, day);
    let time = format!("{:02}:{:02}:{:02}.{:03}:{:03}:{:03}", total_hours, minutes, seconds, millis, micros, nanos);
    
    (date, time)
}

fn days_to_date(mut days: u32) -> (i32, u32, u32) {
    let mut year = 1970;

    // Find the year
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    // Find the month and day
    let mut month = 1;
    while days > 0 {
        let days_in_month = days_in_month(year, month);
        if days < days_in_month {
            break;
        }
        days -= days_in_month;
        month += 1;
    }

    (year, month, days + 1)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unix_nano() {
        let unix_nano = get_unix_nano();
        println!("unix_nano: {}", unix_nano);
        assert!(unix_nano > 0);
    }

    #[test]
    fn test_time_components_from_unix_nano() {
        let unix_nano = get_unix_nano();
        let res = convert_unix_nano_to_date_and_time(unix_nano, 9);
        println!("{:?}", res);
        
    }
}