use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day9.txt");

    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut visited_two: Vec<(i32, i32)> = Vec::new();

    let mut rope_location_array = [(0, 0); 10];

    for instruction in file_contents {
        let instruction_details: Vec<&str> = instruction.split(" ").collect();
        let number_of_times: u32 = string_to_number(instruction_details[1].to_string());
        for _ in 1..=number_of_times {
            match instruction_details[0] {
                "R" => move_rope(1, 0, &mut rope_location_array),
                "L" => move_rope(-1, 0, &mut rope_location_array),
                "U" => move_rope(0, 1, &mut rope_location_array),
                "D" => move_rope(0, -1, &mut rope_location_array),
                _ => panic!("Unexpected direction"),
            }
            if !visited.contains(&rope_location_array[1]) {
                visited.push(rope_location_array[1])
            }
            if !visited_two.contains(&rope_location_array[9]) {
                visited_two.push(rope_location_array[9])
            }
        }
    }

    println!("Part One {}", visited.len());
    println!("Part Two {}", visited_two.len());
}

fn move_rope(x_move: i32, y_move: i32, rope_location_array: &mut [(i32, i32); 10]) {
    rope_location_array[0].0 += x_move;
    rope_location_array[0].1 += y_move;

    for knot_number in 0..=8 {
        if (rope_location_array[knot_number + 1].0 - rope_location_array[knot_number].0) > 1
            && (rope_location_array[knot_number + 1].1 - rope_location_array[knot_number].1) > 1
        {
            // x difference is greater than 1 - Tail is too far right
            // y difference is greater than 1 - tail is too far up

            rope_location_array[knot_number + 1].0 = rope_location_array[knot_number].0.clone() + 1;
            rope_location_array[knot_number + 1].1 = rope_location_array[knot_number].1.clone() + 1;
        } else if (rope_location_array[knot_number + 1].0 - rope_location_array[knot_number].0) < -1
            && (rope_location_array[knot_number + 1].1 - rope_location_array[knot_number].1) > 1
        {
            // x difference is less than -1 - Tail is too far left
            // y difference is greater than 1 - tail is too far up

            rope_location_array[knot_number + 1].0 = rope_location_array[knot_number].0.clone() - 1;
            rope_location_array[knot_number + 1].1 = rope_location_array[knot_number].1.clone() + 1;
        } else if (rope_location_array[knot_number + 1].0 - rope_location_array[knot_number].0) > 1
            && (rope_location_array[knot_number + 1].1 - rope_location_array[knot_number].1) < -1
        {
            // x difference is greater than 1 - Tail is too far right
            // y difference is less than -1 - tail is too far down

            rope_location_array[knot_number + 1].0 = rope_location_array[knot_number].0.clone() + 1;
            rope_location_array[knot_number + 1].1 = rope_location_array[knot_number].1.clone() - 1;
        } else if (rope_location_array[knot_number + 1].0 - rope_location_array[knot_number].0) < -1
            && (rope_location_array[knot_number + 1].1 - rope_location_array[knot_number].1) < -1
        {
            // x difference is less than -1 - Tail is too far left
            // y difference is less than -1 - tail is too far down

            rope_location_array[knot_number + 1].0 = rope_location_array[knot_number].0.clone() - 1;
            rope_location_array[knot_number + 1].1 = rope_location_array[knot_number].1.clone() - 1;
        }
        if (rope_location_array[knot_number + 1].0 - rope_location_array[knot_number].0) > 1 {
            // x difference is greater than 1 - Tail is too far right

            rope_location_array[knot_number + 1].0 = rope_location_array[knot_number].0.clone() + 1;
            rope_location_array[knot_number + 1].1 = rope_location_array[knot_number].1.clone();
        } else if (rope_location_array[knot_number + 1].0 - rope_location_array[knot_number].0) < -1
        {
            // x difference is less than -1 - Tail is too far left

            rope_location_array[knot_number + 1].0 = rope_location_array[knot_number].0.clone() - 1;
            rope_location_array[knot_number + 1].1 = rope_location_array[knot_number].1.clone();
        } else if (rope_location_array[knot_number + 1].1 - rope_location_array[knot_number].1) > 1
        {
            // y difference is greater than 1 - tail is too far up

            rope_location_array[knot_number + 1].0 = rope_location_array[knot_number].0.clone();
            rope_location_array[knot_number + 1].1 = rope_location_array[knot_number].1.clone() + 1;
        } else if (rope_location_array[knot_number + 1].1 - rope_location_array[knot_number].1) < -1
        {
            // y difference is less than -1 - tail is too far down

            rope_location_array[knot_number + 1].0 = rope_location_array[knot_number].0.clone();
            rope_location_array[knot_number + 1].1 = rope_location_array[knot_number].1.clone() - 1;
        }
    }
}
