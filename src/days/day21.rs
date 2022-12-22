use std::collections::HashMap;

use regex::Regex;

use crate::helpers::{
    file_helper::read_file_to_string_vector,
    parse_helper::{get_match, string_to_number_i32},
};

pub fn run_day() {
    let file_contents = read_file_to_string_vector("day21.txt");

    let mut monkeys = get_monkeys(file_contents);

    let root_monkey: i64 = play_with_monkeys("root".to_string(), &mut monkeys);

    println!("Part 1 Score: {}", root_monkey);

    let monkey_equation = monkeys.get(&"root".to_string()).unwrap().1.clone();
    let (monkey_one, monkey_two) = get_next_monkeys(monkey_equation);

    let monkey_one_has_human = check_for_human(monkey_one.clone(), monkeys.clone());

    if monkey_one_has_human {
        reverse_monkey(
            monkey_one.clone(),
            monkeys.get(&monkey_two).unwrap().0,
            &mut monkeys,
        );
    } else {
        reverse_monkey(
            monkey_two.clone(),
            monkeys.get(&monkey_one).unwrap().0,
            &mut monkeys,
        );
    }

    println!(
        "Part 2 Score: {}",
        monkeys.get(&"humn".to_string()).unwrap().0
    );
}

fn reverse_monkey(
    current_monkey: String,
    target_value: i64,
    monkeys: &mut HashMap<String, (i64, String)>,
) {
    if current_monkey == "humn".to_string() {
        monkeys.get_mut(&current_monkey).unwrap().0 = target_value;
        return;
    }
    let monkey_equation = monkeys.get(&current_monkey).unwrap().1.clone();

    let (monkey_one, monkey_two) = get_next_monkeys(monkey_equation);

    let monkey_one_has_human = check_for_human(monkey_one.clone(), monkeys.clone());

    let monkey_action_match = Regex::new(r"[+\-*/]").unwrap();
    if monkey_one_has_human {
        match get_match(
            monkey_action_match,
            monkeys.clone().get(&current_monkey).unwrap().1.clone(),
        )
        .as_str()
        {
            "+" => reverse_monkey(
                monkey_one.clone(),
                target_value - monkeys.get(&monkey_two).unwrap().0,
                monkeys,
            ),
            "-" => reverse_monkey(
                monkey_one.clone(),
                target_value + monkeys.get(&monkey_two).unwrap().0,
                monkeys,
            ),
            "*" => reverse_monkey(
                monkey_one.clone(),
                target_value / monkeys.get(&monkey_two).unwrap().0,
                monkeys,
            ),
            "/" => reverse_monkey(
                monkey_one.clone(),
                target_value * monkeys.get(&monkey_two).unwrap().0,
                monkeys,
            ),
            _ => panic!("Unexpected action"),
        };
    } else {
        match get_match(
            monkey_action_match,
            monkeys.clone().get(&current_monkey).unwrap().1.clone(),
        )
        .as_str()
        {
            "+" => reverse_monkey(
                monkey_two.clone(),
                target_value - monkeys.get(&monkey_one).unwrap().0,
                monkeys,
            ),
            "-" => reverse_monkey(
                monkey_two.clone(),
                monkeys.get(&monkey_one).unwrap().0 - target_value,
                monkeys,
            ),
            "*" => reverse_monkey(
                monkey_two.clone(),
                target_value / monkeys.get(&monkey_one).unwrap().0,
                monkeys,
            ),
            "/" => reverse_monkey(
                monkey_two.clone(),
                monkeys.get(&monkey_one).unwrap().0 / target_value,
                monkeys,
            ),
            _ => panic!("Unexpected action"),
        };
    }
}

fn check_for_human(current_monkey: String, monkeys: HashMap<String, (i64, String)>) -> bool {
    if current_monkey == "humn".to_string() {
        true
    } else {
        let monkey_equation = monkeys.get(&current_monkey).unwrap().1.clone();
        if monkey_equation == "".to_string() {
            false
        } else {
            let (monkey_one, monkey_two) = get_next_monkeys(monkey_equation);
            check_for_human(monkey_one, monkeys.clone())
                || check_for_human(monkey_two, monkeys.clone())
        }
    }
}

fn play_with_monkeys(starting_monkey: String, monkeys: &mut HashMap<String, (i64, String)>) -> i64 {
    if monkeys.get(&starting_monkey).unwrap().0 != -1 {
        return monkeys.get(&starting_monkey).unwrap().0;
    }

    // Equation monkey
    let monkey_equation = monkeys.get(&starting_monkey).unwrap().1.clone();
    let monkey_action_match = Regex::new(r"[+\-*/]").unwrap();

    let (monkey_one, monkey_two) = get_next_monkeys(monkey_equation);

    let monkey_number: i64;

    match get_match(
        monkey_action_match,
        monkeys.clone().get(&starting_monkey).unwrap().1.clone(),
    )
    .as_str()
    {
        "+" => {
            monkey_number =
                play_with_monkeys(monkey_one, monkeys) + play_with_monkeys(monkey_two, monkeys)
        }
        "-" => {
            monkey_number =
                play_with_monkeys(monkey_one, monkeys) - play_with_monkeys(monkey_two, monkeys)
        }
        "*" => {
            monkey_number =
                play_with_monkeys(monkey_one, monkeys) * play_with_monkeys(monkey_two, monkeys)
        }
        "/" => {
            monkey_number =
                play_with_monkeys(monkey_one, monkeys) / play_with_monkeys(monkey_two, monkeys)
        }
        _ => panic!("Unexpected action"),
    };

    monkeys.get_mut(&starting_monkey).unwrap().0 = monkey_number;

    monkey_number
}

fn get_next_monkeys(monkey_equation: String) -> (String, String) {
    let monkey_name_match = Regex::new("[a-z]+").unwrap();
    let mut monkey_name_result = monkey_name_match.captures_iter(&monkey_equation);
    let monkey_one = monkey_name_result
        .next()
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string();
    let monkey_two = monkey_name_result
        .next()
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string();
    (monkey_one, monkey_two)
}

fn get_monkeys(file_contents: Vec<String>) -> HashMap<String, (i64, String)> {
    let mut monkeys: HashMap<String, (i64, String)> = HashMap::new();
    for line in file_contents {
        let monkey_name_match = Regex::new("[a-z]+").unwrap();
        let monkey_name = get_match(monkey_name_match, line.clone());

        let monkey_number_match = Regex::new(r"[\d]+").unwrap();

        let regex_result = monkey_number_match.captures_iter(&line);
        let mut valve_match_result = regex_result;
        let monkey_number_option = valve_match_result.next();

        let monkey_number;
        let monkey_equation;
        if let Some(value) = monkey_number_option {
            monkey_number = string_to_number_i32(value.get(0).unwrap().as_str().to_string());
            monkey_equation = String::new()
        } else {
            monkey_number = -1;
            let monkey_equation_match = Regex::new(r"[a-z]+[+\-*/ ]+[a-z]+").unwrap();
            monkey_equation = get_match(monkey_equation_match, line.clone());
        }

        monkeys.insert(monkey_name, (monkey_number as i64, monkey_equation));
    }
    monkeys
}
