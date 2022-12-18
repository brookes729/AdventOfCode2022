use std::collections::HashMap;

use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number_i32};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day18.txt");

    let mut droplets: HashMap<(i32, i32, i32), i32> = HashMap::new();
    let directions = vec![
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];
    let mut cube = ((9999, -9999), (9999, -9999), (9999, -9999));

    for line in file_contents {
        check_touching_sides(line, &directions, &mut droplets, &mut cube);
    }

    let surface_area: i32 = droplets.values().collect::<Vec<&i32>>().into_iter().sum();

    let water = find_water(&directions, &mut droplets, &mut cube);
    let water_surface_area: i32 = water.values().collect::<Vec<&i32>>().into_iter().sum();

    println!("Part 1: {}", surface_area);
    println!("Part 1: {}", water_surface_area);
}

fn find_water(
    directions: &Vec<(i32, i32, i32)>,
    droplets: &mut HashMap<(i32, i32, i32), i32>,
    cube: &mut ((i32, i32), (i32, i32), (i32, i32)),
) -> HashMap<(i32, i32, i32), i32> {
    let mut water: HashMap<(i32, i32, i32), i32> = HashMap::new();
    let mut options = vec![((cube.0).0 - 1, (cube.1).0 - 1, (cube.2).0 - 1)];

    while options.len() > 0 {
        let (x, y, z) = options.pop().unwrap();

        let mut exposed_sides = 6;
        for direction in directions.clone() {
            if (direction.0 < 0 && x + direction.0 < (cube.0).0 - 1)
                || (direction.0 > 0 && x + direction.0 > (cube.0).1 + 1)
                || (direction.1 < 0 && y + direction.1 < (cube.1).0 - 1)
                || (direction.1 > 0 && y + direction.1 > (cube.1).1 + 1)
                || (direction.2 < 0 && z + direction.2 < (cube.2).0 - 1)
                || (direction.2 > 0 && z + direction.2 > (cube.2).1 + 1)
            {
                exposed_sides -= 1;
            } else if water.contains_key(&(x + direction.0, y + direction.1, z + direction.2)) {
                *water
                    .get_mut(&(x + direction.0, y + direction.1, z + direction.2))
                    .unwrap() -= 1;
                exposed_sides -= 1;
            } else if !droplets.contains_key(&(x + direction.0, y + direction.1, z + direction.2))
                && !options.contains(&(x + direction.0, y + direction.1, z + direction.2))
            {
                options.push((x + direction.0, y + direction.1, z + direction.2));
            }
        }
        water.insert((x, y, z), exposed_sides);
    }

    water
}

fn check_touching_sides(
    line: String,
    directions: &Vec<(i32, i32, i32)>,
    droplets: &mut HashMap<(i32, i32, i32), i32>,
    cube: &mut ((i32, i32), (i32, i32), (i32, i32)),
) {
    let position = line
        .split(",")
        .into_iter()
        .map(|x| string_to_number_i32(x.to_string()))
        .collect::<Vec<_>>();
    let mut exposed_sides = 6;
    let x = position.get(0).unwrap();
    let y = position.get(1).unwrap();
    let z = position.get(2).unwrap();
    for direction in directions.clone() {
        if droplets.contains_key(&(x + direction.0, y + direction.1, z + direction.2)) {
            *droplets
                .get_mut(&(x + direction.0, y + direction.1, z + direction.2))
                .unwrap() -= 1;
            exposed_sides -= 1;
        }
    }
    droplets.insert((x.clone(), y.clone(), z.clone()), exposed_sides);
    if x < &(cube.0).0 {
        (cube.0).0 = x.clone();
    }
    if x > &(cube.0).1 {
        (cube.0).1 = x.clone();
    }
    if y < &(cube.1).0 {
        (cube.1).0 = y.clone();
    }
    if y > &(cube.1).1 {
        (cube.1).1 = y.clone();
    }
    if z < &(cube.2).0 {
        (cube.2).0 = z.clone();
    }
    if z > &(cube.2).1 {
        (cube.2).1 = z.clone();
    }
}
