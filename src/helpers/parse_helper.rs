pub fn string_to_number(input: String) -> u32 {
    let output = input.parse::<u32>();
    match output {
        Ok(value) => value,
        Err(err) => panic!("Invalid value to parse {}: {}", input, err),
    }
}
