pub mod day0;
pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

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
        6 => day6::run_day(),
        7 => day7::run_day(),
        8 => day8::run_day(),
        9 => day9::run_day(),
        10 => day10::run_day(),
        11 => day11::run_day(),
        12 => day12::run_day(),
        13 => day13::run_day(),
        14 => day14::run_day(),
        15 => day15::run_day(),
        16 => day16::run_day(),
        17 => day17::run_day(),
        18 => day18::run_day(),
        19 => day19::run_day(),
        20 => day20::run_day(),
        21 => day21::run_day(),
        22 => day21::run_day(),
        _ => println!("We've not go to that day yet"),
    }
}
