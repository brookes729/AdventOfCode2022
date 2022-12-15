use regex::Regex;

use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number_i32};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day15.txt");

    let part_one_observed_line = 2000000;
    let mut beacons_on_observed: Vec<i32> = Vec::new();
    let observed_line_sections: Vec<(i32, i32)> = Vec::new();

    let row_count = part_one_count(
        file_contents,
        &mut beacons_on_observed,
        part_one_observed_line,
        observed_line_sections,
    );

    println!("Part 1: {}", row_count - beacons_on_observed.len());

    let file_contents = read_file_to_string_vector("day15.txt");
    let part_two = part_two_missing(file_contents);

    println!(
        "Part 2: x={} y={} value = {}",
        part_two.0,
        part_two.1,
        (4000000 * part_two.0 as i64) + part_two.1 as i64
    );
}

fn part_two_missing(file_contents: Vec<String>) -> (i32, i32) {
    let mut sensors: Vec<(i32, i32, i32)> = Vec::new();
    let mut beacons: Vec<(i32, i32)> = Vec::new();
    for line in file_contents {
        //Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let regex = Regex::new(r"(?m)[-\d]+").unwrap();
        let mut regex_result = regex.captures_iter(&line);
        let sensor = (
            get_next_number(&mut regex_result),
            get_next_number(&mut regex_result),
        );
        let beacon = (
            get_next_number(&mut regex_result),
            get_next_number(&mut regex_result),
        );
        let manhattan_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

        beacons.push(beacon);
        sensors.push((sensor.0, sensor.1, manhattan_distance))
    }
    // brute force is a no go
    // can't work out everything that is hit so must base on lines

    let mut potential_beacons: Vec<(i32, i32)> = Vec::new();
    let area_of_beacons = 4000000;

    for sensor in sensors.clone() {
        for i in 0..=sensor.2 {
            if sensor.0 + i >= 0
                && sensor.0 + i <= area_of_beacons
                && sensor.1 + sensor.2 + 1 - i >= 0
                && sensor.1 + sensor.2 + 1 - i <= area_of_beacons
            {
                potential_beacons.push((sensor.0 + i, sensor.1 + sensor.2 + 1 - i));
            }
        }

        for i in 0..=sensor.2 {
            if sensor.0 - 1 - i >= 0
                && sensor.0 - 1 - i <= area_of_beacons
                && sensor.1 + sensor.2 - i >= 0
                && sensor.1 + sensor.2 - i <= area_of_beacons
            {
                potential_beacons.push((sensor.0 - 1 - i, sensor.1 + sensor.2 - i));
            }
        }

        for i in 0..=sensor.2 {
            if sensor.0 + sensor.2 + 1 - i >= 0
                && sensor.0 + sensor.2 + 1 - i <= area_of_beacons
                && sensor.1 - i >= 0
                && sensor.1 - i <= area_of_beacons
            {
                potential_beacons.push((sensor.0 + sensor.2 + 1 - i, sensor.1 - i));
            }
        }

        for i in 0..=sensor.2 {
            if sensor.0 - sensor.2 + i >= 0
                && sensor.0 - sensor.2 + i <= area_of_beacons
                && sensor.1 - 1 - i >= 0
                && sensor.1 - 1 - i <= area_of_beacons
            {
                potential_beacons.push((sensor.0 - sensor.2 + i, sensor.1 - 1 - i));
            }
        }
    }

    for beacon in potential_beacons {
        let mut beacon_found = true;
        for sensor in sensors.clone() {
            if (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs() <= sensor.2 {
                beacon_found = false;
                break;
            }
        }
        if beacon_found {
            return beacon;
        }
    }
    (0, 0)
}

fn part_one_count(
    file_contents: Vec<String>,
    beacons_on_observed: &mut Vec<i32>,
    part_one_observed_line: i32,
    mut observed_line_sections: Vec<(i32, i32)>,
) -> usize {
    for line in file_contents {
        //Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let regex = Regex::new(r"(?m)[-\d]+").unwrap();
        let mut regex_result = regex.captures_iter(&line);
        let sensor = (
            get_next_number(&mut regex_result),
            get_next_number(&mut regex_result),
        );
        let beacon = (
            get_next_number(&mut regex_result),
            get_next_number(&mut regex_result),
        );
        if !beacons_on_observed.contains(&beacon.0) && beacon.1 == part_one_observed_line {
            beacons_on_observed.push(beacon.0);
        }
        let manhattan_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let distance_to_observed: i32;
        if sensor.1 <= part_one_observed_line
            && sensor.1 + manhattan_distance >= part_one_observed_line
        {
            distance_to_observed = part_one_observed_line - sensor.1;
        } else if sensor.1 >= part_one_observed_line
            && sensor.1 - manhattan_distance <= part_one_observed_line
        {
            distance_to_observed = sensor.1 - part_one_observed_line;
        } else {
            continue;
        }

        observed_line_sections.push((
            sensor.0 - (manhattan_distance - distance_to_observed),
            sensor.0 + (manhattan_distance - distance_to_observed),
        ))
    }
    observed_line_sections.sort_by(|a, b| b.1.cmp(&a.1));
    let largest_value = observed_line_sections[0].1;
    observed_line_sections.sort_by(|a, b| a.0.cmp(&b.0));
    let smallest_value = observed_line_sections[0].0;
    let mut row_count = 0;
    for x in smallest_value..=largest_value {
        for line_section in &observed_line_sections {
            if x >= line_section.0 && x <= line_section.1 {
                row_count += 1;
                break;
            }
        }
    }
    row_count
}

fn get_next_number(regex_result: &mut regex::CaptureMatches) -> i32 {
    string_to_number_i32(
        regex_result
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_string(),
    )
}
