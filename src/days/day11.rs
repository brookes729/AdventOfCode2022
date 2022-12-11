use std::collections::VecDeque;

use crate::helpers::{file_helper::read_file_to_string_vector, parse_helper::string_to_number_i32};

pub fn run_day() {
    let mut monkeys = create_monkeys();
    let mut monkey_inspections: Vec<i32> = Vec::new();

    let mut stress_reducer: i64 = 1;
    for monkey_index in 0..=monkeys.len() - 1 {
        stress_reducer *= monkeys[monkey_index].test;
    }

    for _ in 1..=20 {
        for monkey_index in 0..=monkeys.len() - 1 {
            while monkeys.get(monkey_index).unwrap().monkey_items.len() > 0 {
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let (item, new_monkey_number) = monkey.throw_item(true, stress_reducer);

                let new_monkey = monkeys.get_mut((new_monkey_number) as usize).unwrap();
                new_monkey.monkey_items.push_back(item);

                let monkeys_inspection = monkey_inspections.get_mut(monkey_index);
                match monkeys_inspection {
                    Some(mut_val) => {
                        *mut_val += 1;
                    }
                    None => {
                        monkey_inspections.push(1);
                    }
                }
            }
        }
    }
    let mut round_20 = monkey_inspections.clone();
    round_20.sort_by(|a, b| b.cmp(a));

    println!("Part 1 {}", round_20[0] * round_20[1]);

    // part 2
    let mut monkeys = create_monkeys();
    let mut monkey_inspections: Vec<i32> = Vec::new();
    for _ in 1..=10000 {
        for monkey_index in 0..=monkeys.len() - 1 {
            while monkeys.get(monkey_index).unwrap().monkey_items.len() > 0 {
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let (item, new_monkey_number) = monkey.throw_item(false, stress_reducer);

                let new_monkey = monkeys.get_mut((new_monkey_number) as usize).unwrap();
                new_monkey.monkey_items.push_back(item);

                let monkeys_inspection = monkey_inspections.get_mut(monkey_index);
                match monkeys_inspection {
                    Some(mut_val) => {
                        *mut_val += 1;
                    }
                    None => {
                        monkey_inspections.push(1);
                    }
                }
            }
        }
    }

    monkey_inspections.sort_by(|a, b| b.cmp(a));

    println!(
        "Part 2 {}",
        (monkey_inspections[0] as i64 * monkey_inspections[1] as i64)
    );
}

fn create_monkeys() -> Vec<Monkey> {
    let mut file_contents = read_file_to_string_vector("day11.txt").into_iter();
    let mut monkeys: Vec<Monkey> = Vec::new();
    loop {
        let mut new_monkey: Monkey = MonkeyingAround::new();
        let monkey_number_line = file_contents.next().unwrap();
        let monkey_number_colon = monkey_number_line.split(" ").last().unwrap().to_string();
        let monkey_number = monkey_number_colon.split(":").into_iter().next().unwrap();
        new_monkey.monkey_number = string_to_number_i32(monkey_number.to_string());
        let items_line = file_contents.next().unwrap();
        let my_items = items_line.split(": ").last().unwrap();
        for item in my_items.split(", ") {
            new_monkey
                .monkey_items
                .push_back(string_to_number_i32(item.to_string()) as i128);
        }
        new_monkey.operation = file_contents
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .to_string();
        new_monkey.test = string_to_number_i32(
            file_contents
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .to_string(),
        ) as i64;
        new_monkey.happy_monkey = string_to_number_i32(
            file_contents
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .to_string(),
        );
        new_monkey.sad_monkey = string_to_number_i32(
            file_contents
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .to_string(),
        );

        monkeys.push(new_monkey);

        if let None = file_contents.next() {
            break;
        }
    }
    monkeys
}

// Starting items: 79, 98
// Operation: new = old * 19
// Test: divisible by 23
//   If true: throw to monkey 2
//   If false: throw to monkey 3
struct Monkey {
    monkey_number: i32,
    monkey_items: VecDeque<i128>,
    operation: String,
    test: i64,
    happy_monkey: i32,
    sad_monkey: i32,
}

trait MonkeyingAround {
    fn new() -> Self;
    fn throw_item(&mut self, calm: bool, stress_reducer: i64) -> (i128, i32);
    fn process_worry(operation: String, worry_value: i128, calm: bool, stress_reducer: i64)
        -> i128;
}

impl MonkeyingAround for Monkey {
    fn new() -> Monkey {
        Monkey {
            monkey_number: 0,
            monkey_items: VecDeque::new(),
            operation: String::new(),
            test: 0,
            happy_monkey: 0,
            sad_monkey: 0,
        }
    }

    fn throw_item(&mut self, calm: bool, stress_reducer: i64) -> (i128, i32) {
        let worry_value = self.monkey_items.pop_front().unwrap();
        let new_worry_value =
            Monkey::process_worry(self.operation.clone(), worry_value, calm, stress_reducer);

        let next_monkey: i32;
        if new_worry_value % (self.test as i128) == 0 {
            next_monkey = self.happy_monkey
        } else {
            next_monkey = self.sad_monkey
        }

        (new_worry_value, next_monkey)
    }
    fn process_worry(
        operation: String,
        worry_value: i128,
        calm: bool,
        stress_reducer: i64,
    ) -> i128 {
        let function = operation.split(" = ").last().unwrap();
        let function_parts: Vec<&str> = function.split(" ").collect();

        let mut new_value: i128;
        if function_parts.get(0).unwrap() == &"old" {
            new_value = worry_value;
        } else {
            new_value = string_to_number_i32(function_parts.get(0).unwrap().to_string()) as i128;
        }
        let second_value: i128;
        if function_parts.get(2).unwrap() == &"old" {
            second_value = worry_value;
        } else {
            second_value = string_to_number_i32(function_parts.get(2).unwrap().to_string()) as i128;
        }

        match function_parts.get(1).unwrap() {
            &"+" => new_value += second_value as i128,
            &"*" => new_value *= second_value as i128,
            _ => panic!("Unknown calc"),
        }

        if calm {
            ((new_value as f64) / 3 as f64).floor() as i128;
        }
        new_value % stress_reducer as i128
    }
}
