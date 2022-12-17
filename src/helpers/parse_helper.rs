use regex::Regex;

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

pub fn get_match(regex: Regex, line: String) -> String {
    let regex_result = regex.captures_iter(&line);
    get_next_match(regex_result)
}

pub fn get_next_match(mut valve_match_result: regex::CaptureMatches) -> String {
    valve_match_result
        .next()
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string()
}
