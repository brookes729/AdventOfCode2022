mod days;
mod helpers;
use helpers::terminal_helper::get_user_input;

fn main() {
    println!("Welcome to Advent of Code 2022 - Merry Christmas!");
    let day = get_user_input("What day would you like to know about? ");
    println!("Here is day {day}");
    days::run_day(day);
}
