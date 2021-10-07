// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
    "Hello, World!"
}

// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
        expected_minutes_in_oven() - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * 2
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}

// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

#![allow(unused)]
const production: u8 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed >= 9 {
        production as f64 * speed as f64 * 0.77
    } else if speed >= 5 {
        production as f64 * speed as f64 * 0.9
    } else {
        production as f64 * speed as f64
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}

fn main() {
    println!("{}", hello());
}


