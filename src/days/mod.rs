pub mod day0;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

// pub fn run_day_by_string(day_string: String) {
//     let day_number = day_string.parse::<u32>().unwrap();
//     run_day(day_number);
// }

pub fn run_day(day_number: u32) {
    match day_number {
        0 => day0::run_day(),
        1 => day1::run_day(),
        2 => day2::run_day(),
        3 => day3::run_day(),
        4 => day4::run_day(),
        5 => day5::run_day(),
        _ => println!("We've not go to that day yet"),
    }
}
