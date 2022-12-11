use regex::Regex;
use std::fs;

const SUPERMODULO: i64 = 9699690;
const ROUNDS: u32 = 10000;

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: [String; 3],
    test_division: i64,
    test_valid: usize,
    test_invalid: usize,
    items_inspected: u32,
}

impl Monkey {
    fn get_operation(&self, item: i64) -> i64 {
        let operator = self.operation[1].as_str();
        let value_1 = if self.operation[0] == "old" {
            item
        } else {
            self.operation[1].parse::<i64>().unwrap()
        };
        let value_2 = if self.operation[2] == "old" {
            item
        } else {
            self.operation[2].parse::<i64>().unwrap()
        };

        let value = match operator {
            "+" => value_1 + value_2,
            "*" => value_1 * value_2,
            "-" => value_1 - value_2,
            "/" => value_1 / value_2,
            _ => 0,
        };

        value % SUPERMODULO
    }
}

fn get_double_point_position(text: &str) -> usize {
    text.chars().position(|c| c == ':').unwrap()
}

fn extract_items(text: &str) -> Vec<i64> {
    let double_point = get_double_point_position(text);

    text[double_point + 1..]
        .split(",")
        .map(|value| value.trim().parse::<i64>().unwrap())
        .collect()
}

fn extract_operation(text: &str) -> [String; 3] {
    let equal = text.chars().position(|c| c == '=').unwrap();

    text[equal + 1..]
        .trim()
        .split(" ")
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .try_into()
        .unwrap()
}

fn extract_test(text: &str) -> i64 {
    let re_test_division = Regex::new(r"Test: divisible by ([0-9]+)").unwrap();

    for cap in re_test_division.captures_iter(text) {
        return cap[1].parse::<i64>().unwrap();
    }

    -1
}

fn extract_test_valid(text: &str) -> usize {
    let re_test_valid = Regex::new(r"If true: throw to monkey ([0-9]+)").unwrap();

    for cap in re_test_valid.captures_iter(text) {
        return cap[1].parse::<usize>().unwrap();
    }

    0
}

fn extract_test_invalid(text: &str) -> usize {
    let re_test_invalid = Regex::new(r"If false: throw to monkey ([0-9]+)").unwrap();

    for cap in re_test_invalid.captures_iter(text) {
        return cap[1].parse::<usize>().unwrap();
    }

    0
}

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let mut monkeys: Vec<Monkey> = Vec::new();

    // Parse the monkeys
    contents.split("Monkey").for_each(|monkey_info| {
        let mut items: Vec<i64> = Vec::new();
        let mut operation: [String; 3] = Default::default();
        let mut test_division: i64 = 0;
        let mut test_valid: usize = 0;
        let mut test_invalid: usize = 0;

        println!("Monkey info: {}", monkey_info);
        monkey_info.lines().for_each(|line| {
            if line.contains("Starting items") {
                items = extract_items(line);
            } else if line.contains("Operation") {
                operation = extract_operation(line);
            } else if line.contains("Test") {
                test_division = extract_test(line);
            } else if line.contains("If true") {
                test_valid = extract_test_valid(line)
            } else if line.contains("If false") {
                test_invalid = extract_test_invalid(line);
            }
        });

        if !monkey_info.is_empty() {
            monkeys.push(Monkey {
                items,
                operation,
                test_division,
                test_valid,
                test_invalid,
                items_inspected: 0,
            });
        }
    });

    // Compute each monkey
    for _ in 0..ROUNDS {

        let mut monkey_number = 0;

        while monkey_number < monkeys.len() {
            let mut counter: usize = 0;
            let items_len = monkeys[monkey_number].items.len();

            while counter < items_len {
                let item = *monkeys[monkey_number].items.get(0).unwrap();
                let operation = monkeys[monkey_number].get_operation(item);

                monkeys[monkey_number].items_inspected += 1;

                if operation % monkeys[monkey_number].test_division == 0 {
                    let test_valid_monkey = monkeys[monkey_number].test_valid;
                    monkeys[test_valid_monkey].items.push(operation);
                } else {
                    let test_invalid_monkey = monkeys[monkey_number].test_invalid;
                    monkeys[test_invalid_monkey].items.push(operation);
                }

                counter += 1;
                monkeys[monkey_number].items.remove(0);
            }

            monkey_number += 1;
        }
    }

    let sum_two_most_active_monkeys = {
        monkeys.sort_by(|monkey_1, monkey_2| monkey_2.items_inspected.cmp(&monkey_1.items_inspected));
        monkeys.get(0).unwrap().items_inspected as u64 * monkeys.get(1).unwrap().items_inspected as u64
    };

    // dbg!("{:?}", monkeys);
    println!("Sum two most active monkeys: {}", sum_two_most_active_monkeys);
}
