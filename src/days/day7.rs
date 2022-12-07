use std::collections::{hash_map::Entry, HashMap, VecDeque};

use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day7.txt");

    let mut directories_size: HashMap<String, u32> = HashMap::new();
    let mut open_directories: VecDeque<String> = VecDeque::from(["".to_string()]);

    for line in file_contents {
        if line == "$ ls" || line.starts_with("dir") {
            continue;
        }
        if line == "$ cd /" {
            open_directories = VecDeque::from(["".to_string()]);
            continue;
        }
        if line == "$ cd .." {
            open_directories.pop_back();
            continue;
        }
        if line.starts_with("$ cd ") {
            let mut new_directory = line.split(" ").last().unwrap().to_string();
            if let Some(parent) = open_directories.back() {
                new_directory = format!("{}/{}", parent, new_directory)
            }
            open_directories.push_back(new_directory);
            continue;
        }
        // size filename
        let file_size = string_to_number(line.split(" ").collect::<Vec<&str>>()[0].to_string());
        for directory in open_directories.clone() {
            match directories_size.entry(directory) {
                Entry::Vacant(e) => {
                    e.insert(file_size);
                }
                Entry::Occupied(mut e) => {
                    *e.get_mut() += file_size;
                }
            }
        }
    }

    let small_directories = directories_size.values().clone().filter(|d| **d <= 100000);
    let part_one: u32 = small_directories.sum();
    println!("Part one: {}", part_one);

    let file_disk_space = 70000000;
    let free_space = file_disk_space - directories_size.get(&"".to_string()).unwrap();
    let required_space = 30000000 - free_space;
    let mut potential_directories: Vec<&u32> = directories_size
        .values()
        .clone()
        .filter(|d| **d >= required_space)
        .collect();

    potential_directories.sort();
    let part_two = potential_directories.get(0).unwrap();
    println!("Part two: {}", part_two);
}
