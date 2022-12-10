use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number_i32};

pub fn run_day() {
    let mut file_contents = read_file_to_string_vector("day10.txt").into_iter();

    let mut running_x = 1;
    let mut addx_value = 0;
    let mut part_one = 0;
    let mut crt_screen: Vec<i32> = Vec::from([0]);

    for cycle in 1..=280 {
        if cycle % 40 == 20 {
            part_one += cycle * running_x;
        }
        if addx_value != 0 {
            running_x += addx_value;
            addx_value = 0;
        } else {
            let instruction_option = file_contents.next();
            match instruction_option {
                None => break,
                Some(instruction) => {
                    if instruction != "noop".to_string() {
                        // not noop so addx V
                        addx_value = string_to_number_i32(
                            instruction.split(" ").collect::<Vec<&str>>()[1].to_string(),
                        )
                    };
                }
            }
        }
        set_pixel(&mut crt_screen, cycle, running_x);
        draw_crt(crt_screen.clone());
    }

    println!();
    println!("Part 1 {}", part_one);
}

fn draw_crt(crt_screen: Vec<i32>) {
    print!("\x1B[2J\x1B[1;1H"); // clear console
    for pixel in 0..=239 {
        if pixel % 40 == 0 {
            println!();
        }
        if crt_screen.contains(&pixel) {
            print!("#")
        } else {
            print!(" ")
        }
    }
}

fn set_pixel(crt_screen: &mut Vec<i32>, pixel: i32, sprite_pos: i32) {
    if (sprite_pos - (pixel % 40)).abs() < 2 {
        if !crt_screen.contains(&pixel) {
            crt_screen.push(pixel)
        }
    }
}
