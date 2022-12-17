use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::helpers::{
    file_helper::read_file_to_string_vector,
    parse_helper::{get_match, string_to_number_i32},
};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day16.txt");

    let mut map_routes: HashMap<String, Vec<String>> = HashMap::new();
    let mut valve_rates: HashMap<String, i32> = HashMap::new();

    for line in file_contents {
        //Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let valve_match = Regex::new(r"[A-Z]{2}").unwrap();
        let mut valve_match_result = valve_match.captures_iter(&line);
        let valve = valve_match_result
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_string();

        let mut next_locations: Vec<String> = Vec::new();
        while let Some(next_valve) = valve_match_result.next() {
            next_locations.push(next_valve.get(0).unwrap().as_str().to_string())
        }
        map_routes.insert(valve.clone(), next_locations);
        let valve_rate_match = Regex::new(r"[\d]+").unwrap();
        let valve_rate = string_to_number_i32(get_match(valve_rate_match, line));

        valve_rates.insert(valve, valve_rate);
    }

    let current_valve = String::from("AA");
    let remaining_time = 30;
    let mut visited: Vec<String> = Vec::new();
    let mut cached_distances: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut cached_results: HashMap<Vec<String>, i32> = HashMap::new();
    find_best_from_point_mark2(
        current_valve,
        &mut visited,
        remaining_time,
        0,
        &map_routes,
        &valve_rates,
        &mut cached_distances,
        &mut cached_results,
    );

    println!("Part 1: {}", cached_results.values().max().unwrap());

    let current_valve = String::from("AA");
    let remaining_time = 26;
    let mut visited: Vec<String> = Vec::new();
    let mut cached_results: HashMap<Vec<String>, i32> = HashMap::new();
    find_best_from_point_mark2(
        current_valve,
        &mut visited,
        remaining_time,
        0,
        &map_routes,
        &valve_rates,
        &mut cached_distances,
        &mut cached_results,
    );

    let mut paired_matching_one: Vec<(HashSet<String>, i32)> = cached_results
        .clone()
        .into_iter()
        .map(|x| (x.0.into_iter().collect::<HashSet<String>>(), x.1))
        .collect();
    paired_matching_one.sort_by(|a, b| b.1.cmp(&a.1));
    let paired_matching_two = paired_matching_one.clone();
    let mut part_two = 0;

    for first_set in paired_matching_one {
        for second_set in paired_matching_two.clone() {
            if part_two < first_set.1 + second_set.1 {
                if first_set.0.is_disjoint(&second_set.0) {
                    part_two = first_set.1 + second_set.1;
                    break;
                }
            }
        }
        if part_two != 0 {
            break;
        }
    }

    println!("Part 2: {}", part_two);
}

fn find_best_from_point_mark2(
    next_node: String,
    visited: &mut Vec<String>,
    remaining_time: i32,
    released_pressure: i32,
    map_routes: &HashMap<String, Vec<String>>,
    valve_rates: &HashMap<String, i32>,
    cached_distances: &mut HashMap<String, HashMap<String, i32>>,
    cached_results: &mut HashMap<Vec<String>, i32>,
) {
    let times_to_next =
        time_to_next_valve(next_node, remaining_time, &map_routes, cached_distances);
    let potential_flow: HashMap<String, i32> = times_to_next
        .clone()
        .into_iter()
        .filter(|time| {
            valve_rates.get(&time.0).unwrap() > &0 && time.1 > 0 && !visited.contains(&time.0)
        })
        .collect();

    for potential in potential_flow {
        let next_option = potential;
        let mut new_visited = visited.clone();
        new_visited.push(next_option.0.clone());
        new_visited.sort();
        let new_remaining_time = next_option.1 - 1;
        let flow = new_remaining_time * valve_rates.get(&next_option.0).unwrap();

        if cached_results.contains_key(&new_visited) {
            *cached_results.get_mut(&new_visited).unwrap() = std::cmp::max(
                released_pressure + flow,
                *cached_results.get(&new_visited).unwrap(),
            );
        } else {
            cached_results.insert(new_visited.clone(), released_pressure + flow);
        }

        find_best_from_point_mark2(
            next_option.0.clone(),
            &mut new_visited,
            new_remaining_time,
            released_pressure + flow,
            map_routes,
            valve_rates,
            cached_distances,
            cached_results,
        );
    }
}

fn time_to_next_valve(
    current: String,
    remaining_time: i32,
    map_routes: &HashMap<String, Vec<String>>,
    cached_distances: &mut HashMap<String, HashMap<String, i32>>,
) -> HashMap<String, i32> {
    if cached_distances.contains_key(&current) {
        let result: HashMap<String, i32> = cached_distances
            .get(&current)
            .unwrap()
            .clone()
            .into_iter()
            .map(|distance| (distance.0, remaining_time - distance.1))
            .collect();
        return result;
    }

    let mut time_to_get: HashMap<String, i32> = HashMap::new();
    time_to_get.insert(current.clone(), remaining_time);
    let mut next_visits: Vec<(String, i32)> = Vec::new();
    next_visits.push((current.clone(), remaining_time));

    while next_visits.len() > 0 {
        let next_visit = next_visits.pop().unwrap();

        for next_valve in map_routes.get(&next_visit.0).unwrap() {
            if !time_to_get.contains_key(next_valve) {
                time_to_get.insert(next_valve.clone(), next_visit.clone().1 - 1);
                next_visits.push((next_valve.clone(), next_visit.1 - 1));
            } else {
                if time_to_get.get(next_valve).unwrap() < &next_visit.clone().1 {
                    *time_to_get.get_mut(next_valve).unwrap() = next_visit.clone().1 - 1;
                    next_visits.push((next_valve.clone(), next_visit.1 - 1));
                }
            }
        }
    }

    cached_distances.insert(
        current,
        time_to_get
            .clone()
            .into_iter()
            .map(|distance| (distance.0, remaining_time - distance.1))
            .collect(),
    );

    time_to_get
}
