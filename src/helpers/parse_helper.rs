pub fn string_to_number(input: String) -> u32 {
    let output = input.parse::<u32>();
    match output {
        Ok(value) => value,
        Err(err) => panic!("Invalid value to parse {}: {}", input, err),
    }
}
pub fn string_to_number_i32(input: String) -> i32 {
    let output = input.parse::<i32>();
    match output {
        Ok(value) => value,
        Err(err) => panic!("Invalid value to parse {}: {}", input, err),
    }
}
