extern crate chrono;
extern crate time;

use chrono::prelude::*;
use time::Duration;

fn exit_with_user_error_code() -> ! { std::process::exit(1) }

fn main() {
    let mut args = std::env::args();

    args.next(); // skip the executable

    let mut text: String;
    let reverse_convert: bool;

    if args.len() == 0 || args.len() > 2 {
        print_usage_and_exit();
    }

    text = args.next().unwrap();
    reverse_convert = match args.next() {
        None => false,
        Some(arg) => {
            let reverse_convert_argument = arg.to_lowercase();
            reverse_convert_argument == "yes" || reverse_convert_argument == "true"
        }
    };

    let split: Vec<&str> = text.split("/").collect();
    if split.len() != 3 {
        print_usage_and_exit();
    }

    let year: i32 = validate_year(split[2]);
    let month: i32 = validate_month(split[0]);
    let day: i32 = validate_day(month, year, split[1]);

    let epoch: Date<Utc> = Utc.ymd(2013, 1, 1);
    let time: Date<Utc> = Utc.ymd(year, month as u32, day as u32);

    let duration: Duration = time.signed_duration_since(epoch);
    let days: i64 = duration.num_seconds() / 60 / 60 / 24;

    // sl time is 100x faster than real time
    let new_days: i64 = if !reverse_convert { days * 100 } else { days / 100 };
    let new_duration = Duration::seconds(new_days * 24 * 60 * 60);

    // 2013 real = 2300 SL, 287 year diff
    let offset = Duration::days(287 * 365 * (if !reverse_convert { 1 } else { -1 }));

    let converted = epoch + new_duration + offset;

    const FORMAT: &str = "%m/%d/%Y";

    let time_formatted = time.format(FORMAT).to_string();
    let converted_formatted = converted.format(FORMAT).to_string();

    if !reverse_convert {
        println!("{} in real time is {} in SL time", time_formatted, converted_formatted);
    } else {
        println!("{} in SL time is {} in real time", time_formatted, converted_formatted);
    }
}

fn print_usage_and_exit() -> ! {
    println!("Usage: sl_time_convert <month>/<day>/<year> [reverse_convert=false]");
    std::process::exit(0);
}

fn validate_year(input: &str) -> i32 { validate_number(input, -2_147_483_648, 2_147_483_647, "Year") }

fn validate_month(input: &str) -> i32 { validate_number(input, 1, 12, "Month") }

const DAYS_IN_MONTHS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn validate_day(month: i32, year: i32, input: &str) -> i32 {
    let days_in_month = get_days_in_month(month, year);
    let day = validate_number(input, 1, days_in_month, "Day");
    return day;
}

fn get_days_in_month(month: i32, year: i32) -> i32 {
    let days_in_month = match month {
        // special handling for february
        2 => {
            if ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => DAYS_IN_MONTHS[(month - 1) as usize],
    };
    days_in_month
}

fn validate_number(input: &str, min: i32, max: i32, input_name: &str) -> i32 {
    match input.parse() {
        Err(_) => {
            println!("{} {} is not a valid number", input_name, input);
            exit_with_user_error_code();
        }
        Ok(parsed) => {
            if parsed < min {
                println!("{} must be at least {}", input_name, min);
                exit_with_user_error_code();
            } else if parsed > max {
                println!("{} must be at most {}", input_name, max);
                exit_with_user_error_code();
            } else {
                return parsed;
            }
        }
    }
}