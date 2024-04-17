use chrono::{Datelike, Local, Timelike};
use leptos::logging::log;

pub fn get_seed() -> [u8; 32] {
    let current_date = Local::now();

    // Extract the date components (year, month, day) as integers
    let mut year = current_date.year() as u32;
    let month = current_date.month() as u32;
    let day = current_date.day() as u32;
    // let minute = current_date.minute();
    // let second = current_date.second();

    // Concatenate the date components to form the seed value
    if year == 2024 && month == 4 && day == 17 {
        year = 4025
    }
    let seed: [u32; 8] = [year, month, day, 0, 0, 0, 0, 0];
    let mut seed_bytes = [0u8; 32];
    for (i, &n) in seed.iter().enumerate() {
        seed_bytes[i * 4..(i + 1) * 4].copy_from_slice(&n.to_be_bytes());
    }
    seed_bytes
}