use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number_i32};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day13.txt");

    let mut left = String::new();
    let mut right: String;
    let mut counter = 0;
    let mut part_one = 0;

    for line in file_contents {
        if line.is_empty() {
            continue;
        }
        if left.is_empty() {
            left = line;
        } else {
            counter += 1;
            right = line;
            if inputs_are_right(left.clone(), right.clone()).unwrap() {
                part_one += counter;
            }
            left = String::new();
        }
    }

    // part 2
    let file_contents = read_file_to_string_vector("day13.txt");
    let marker_one = String::from("[[2]]");
    let marker_two = String::from("[[6]]");
    let mut marker_one_index = 1;
    let mut marker_two_index = 2; // it will be ahead of marker 1
    let mut test_line: String;

    for line in file_contents {
        if line.is_empty() {
            continue;
        }
        test_line = line;
        if inputs_are_right(test_line.clone(), marker_one.clone()).unwrap() {
            marker_one_index += 1;
        }
        if inputs_are_right(test_line.clone(), marker_two.clone()).unwrap() {
            marker_two_index += 1;
        }
    }

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", marker_one_index * marker_two_index);
}

fn inputs_are_right(left: String, right: String) -> Option<bool> {
    let left_clone = left.clone();
    let mut left_iter = left_clone.chars().into_iter();
    let right_clone = right.clone();
    let mut right_iter = right_clone.chars().into_iter();

    let mut right_order: Option<bool>;
    loop {
        let mut left_int = -1;
        let mut right_int = -1;
        let mut left_list = String::new();
        let mut right_list = String::new();
        let mut left_empty = false;
        let mut right_empty = false;

        get_elements(
            &mut left_iter,
            &mut left_list,
            &mut left_int,
            &mut left_empty,
        );
        get_elements(
            &mut right_iter,
            &mut right_list,
            &mut right_int,
            &mut right_empty,
        );

        if left_empty {
            if right_empty {
                right_order = None;
                break;
            } else {
                right_order = Some(true);
                break;
            }
        } else if right_empty {
            right_order = Some(false);
            break;
        } else if left_int != -1 && right_int != -1 {
            //two numbers, if left is less right order, if equal loop again, if right is bigger break
            if left_int < right_int {
                right_order = Some(true);
                break;
            } else if right_int < left_int {
                right_order = Some(false);
                break;
            }
        } else if left_int != -1 && !right_list.is_empty() {
            // left is a number right is a list
            left_list = left_int.to_string();
            right_order = inputs_are_right(left_list.clone(), right_list.clone());
            if let Some(_) = right_order {
                break;
            }
        } else if right_int != -1 && !left_list.is_empty() {
            // left is a list right is a number
            right_list = right_int.to_string();
            right_order = inputs_are_right(left_list.clone(), right_list.clone());

            if let Some(_) = right_order {
                break;
            }
        } else if left_int != -1 && right_int == -1 {
            // left is a number right is an empty list

            right_order = Some(false);
            break;
        } else if right_int != -1 && left_int == -1 {
            // left is an empty list right is a number

            right_order = Some(true);
            break;
        } else if !left_list.is_empty() && !right_list.is_empty() {
            // left is a list right is a list
            right_order = inputs_are_right(left_list.clone(), right_list.clone());

            if let Some(_) = right_order {
                break;
            }
        } else if left_list.is_empty() {
            // left is an empty list
            if !right_list.is_empty() {
                right_order = Some(true);
                break;
            }
        } else if right_list.is_empty() {
            right_order = Some(false);
            break;
        } else {
            println!("Problem occured");
        }
    }

    right_order
}

fn get_elements(
    iter: &mut std::str::Chars,
    list: &mut String,
    number: &mut i32,
    is_empty: &mut bool,
) {
    if let Some(left_value) = iter.next() {
        let sub_char: char;
        if left_value == ',' {
            sub_char = iter.next().unwrap();
        } else {
            sub_char = left_value;
        }
        if sub_char == '[' {
            get_sub_list(iter, list);
        } else {
            get_number(iter, sub_char, number);
        }
    } else {
        *is_empty = true;
    }
}

fn get_number(iter: &mut std::str::Chars, starting_point: char, number: &mut i32) {
    // not a list so a number
    let mut number_string = starting_point.to_string();
    loop {
        if let Some(numeric_value) = iter.next() {
            if numeric_value == ',' {
                break;
            } else {
                // still a number
                number_string.push(numeric_value)
            }
        } else {
            break;
        }
    }
    *number = string_to_number_i32(number_string)
}

fn get_sub_list(iter: &mut std::str::Chars, sub_list: &mut String) {
    // need to find our matching one
    let mut bracket_count = 0;
    loop {
        if let Some(sub_value) = iter.next() {
            if sub_value == '[' {
                // this is taking us a level deeper
                bracket_count += 1;
            } else if sub_value == ']' {
                if bracket_count == 0 {
                    break;
                } else {
                    bracket_count -= 1;
                }
            }
            sub_list.push(sub_value);
        } else {
            break;
        }
    }
}
