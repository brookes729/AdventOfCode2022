use std::collections::HashMap;

use regex::Regex;

use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number_i32};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day19.txt");

    let mut blueprints: HashMap<i32, Vec<(i32, i32, i32)>> = HashMap::new();
    let mut part_one = 0;
    let mut part_two = 1;

    for line in file_contents {
        let number_regex = Regex::new(r"[\d]+").unwrap();
        let mut number_matches = number_regex.captures_iter(&line);
        let id = get_next_number(&mut number_matches);
        let ore_robot_cost = get_next_number(&mut number_matches);
        let clay_robot_cost = get_next_number(&mut number_matches);
        let obsidian_robot_cost_ore = get_next_number(&mut number_matches);
        let obsidian_robot_cost_clay = get_next_number(&mut number_matches);
        let geode_robot_cost_ore = get_next_number(&mut number_matches);
        let geode_robot_cost_obsidian = get_next_number(&mut number_matches);

        let blueprint = vec![
            (ore_robot_cost, 0, 0),
            (clay_robot_cost, 0, 0),
            (obsidian_robot_cost_ore, obsidian_robot_cost_clay, 0),
            (geode_robot_cost_ore, 0, geode_robot_cost_obsidian),
        ];

        blueprints.insert(id, blueprint.clone());

        let mut cached_results = HashMap::new();
        part_one += id
            * best_robot_strategy(
                vec![1, 0, 0, 0],
                (0, 0, 0, 0),
                24,
                id,
                blueprint.clone(),
                &mut cached_results,
            )
            .3;

        if id < 4 {
            part_two *= best_robot_strategy(
                vec![1, 0, 0, 0],
                (0, 0, 0, 0),
                32,
                id,
                blueprint.clone(),
                &mut cached_results,
            )
            .3;
        }
    }

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}

fn best_robot_strategy(
    robots: Vec<i32>,
    resources: (i32, i32, i32, i32),
    time_remaining: i32,
    blueprint_id: i32,
    blueprint: Vec<(i32, i32, i32)>,
    cached_results: &mut HashMap<(i32, i32, Vec<i32>, (i32, i32, i32, i32)), (i32, i32, i32, i32)>,
) -> (i32, i32, i32, i32) {
    if cached_results.contains_key(&(blueprint_id, time_remaining, robots.clone(), resources)) {
        return cached_results
            .get(&(blueprint_id, time_remaining, robots.clone(), resources))
            .unwrap()
            .clone();
    }

    let mut best_resources = (0, 0, 0, 0);
    // doing nothing will result in
    let mut new_resources = resources.clone();
    new_resources.0 += time_remaining * robots[0];
    new_resources.1 += time_remaining * robots[1];
    new_resources.2 += time_remaining * robots[2];
    new_resources.3 += time_remaining * robots[3];

    if new_resources.3 >= best_resources.3 {
        best_resources = new_resources.clone();
    }

    // keep a track of the robot, first is 0, increase at end of loop
    let mut robot_type = 0;
    for potential_robot in blueprint.clone() {
        let needed = is_robot_needed_and_buildable(
            potential_robot,
            robot_type,
            &blueprint,
            time_remaining,
            &robots,
            resources,
        );
        if !needed {
            robot_type += 1;
            continue;
        }

        let time_needed = get_time_to_make(potential_robot, resources, &robots);

        if time_needed < time_remaining {
            // we have the time to build it!
            // How many resources will we get in the mean time?
            let mut new_resources = resources.clone();
            new_resources.0 += (time_needed * robots[0]) - potential_robot.0;
            new_resources.1 += (time_needed * robots[1]) - potential_robot.1;
            new_resources.2 += (time_needed * robots[2]) - potential_robot.2;
            new_resources.3 += time_needed * robots[3];

            let mut new_robots = robots.clone();

            *new_robots.get_mut(robot_type).unwrap() += 1;

            let resulting_resources = best_robot_strategy(
                new_robots,
                new_resources,
                time_remaining - time_needed,
                blueprint_id,
                blueprint.clone(),
                cached_results,
            );

            if resulting_resources.3 >= best_resources.3 {
                best_resources = resulting_resources.clone();
            }
        }

        robot_type += 1;
    }

    cached_results.insert(
        (blueprint_id, time_remaining, robots.clone(), resources),
        best_resources,
    );

    best_resources
}

fn get_time_to_make(
    potential_robot: (i32, i32, i32),
    resources: (i32, i32, i32, i32),
    robots: &Vec<i32>,
) -> i32 {
    let mut time_needed = 0;
    if potential_robot.0 > 0 {
        time_needed = ((potential_robot.0 - resources.0) as f32 / robots[0] as f32).ceil() as i32;
    }
    if potential_robot.1 > 0 {
        time_needed = std::cmp::max(
            time_needed,
            ((potential_robot.1 - resources.1) as f32 / robots[1] as f32).ceil() as i32,
        );
    }
    if potential_robot.2 > 0 {
        time_needed = std::cmp::max(
            time_needed,
            ((potential_robot.2 - resources.2) as f32 / robots[2] as f32).ceil() as i32,
        );
    }
    // don't forget the minute to build, even with all resources in stock
    time_needed = std::cmp::max(1, time_needed + 1);
    time_needed
}

fn is_robot_needed_and_buildable(
    potential_robot: (i32, i32, i32),
    robot_type: usize,
    blueprint: &Vec<(i32, i32, i32)>,
    time_remaining: i32,
    robots: &Vec<i32>,
    resources: (i32, i32, i32, i32),
) -> bool {
    let buildable = (potential_robot.0 == 0 || robots[0] > 0 || resources.0 > potential_robot.0)
        && (potential_robot.1 == 0 || robots[1] > 0 || resources.1 > potential_robot.1)
        && (potential_robot.2 == 0 || robots[2] > 0 || resources.2 > potential_robot.2);

    let mut needed = robot_type == 3;
    for robot_plan in blueprint.clone() {
        match robot_type {
            0 => {
                needed = robot_plan.0 * time_remaining > (robots[0] * time_remaining) + resources.0
            }
            1 => {
                needed = robot_plan.1 * time_remaining > (robots[1] * time_remaining) + resources.1
            }
            2 => {
                needed = robot_plan.2 * time_remaining > (robots[2] * time_remaining) + resources.2
            }
            _ => needed = true,
        }
        if needed {
            break;
        }
    }
    needed && buildable
}

fn get_next_number(number_matches: &mut regex::CaptureMatches) -> i32 {
    string_to_number_i32(
        number_matches
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_string(),
    )
}
