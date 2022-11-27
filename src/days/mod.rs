pub mod day0;

pub fn run_day(day_string: String) {
    let day_number = day_string.parse::<i8>().unwrap();
    match day_number {
        0 => {
            println!("we're good to go");
            day0::run_day()
        }
        _ => println!("We've not go to that day yet"),
    }
}
