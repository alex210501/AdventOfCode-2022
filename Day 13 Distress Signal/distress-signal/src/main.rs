use std::fs;

use json::{array, JsonValue};
extern crate json;
// use json;

fn compute_number(pair_1: &JsonValue, pair_2: &JsonValue) -> bool {
    // println!("compute number: {} - {}", pair_1, pair_2);
    // println!("compute number: {} - {}", pair_1.is_number(), pair_2.is_number());
    let number_1 = pair_1.as_u32().unwrap();
    let number_2 = pair_2.as_u32().unwrap();

    number_1 <= number_2
}

fn compute_pair(pair_1: &JsonValue, pair_2: &JsonValue) -> bool {
    println!("1: {}\n2: {}", pair_1, pair_2);

    if pair_1.is_number() && pair_1.is_number() {
        return compute_number(&pair_1, &pair_1);
    }

    for i in 0..pair_1.len() {
        let value_1 = pair_1[i].clone();
        let value_2 = pair_2[i].clone();

        if value_1.is_number() && value_2.is_number() {
            if !compute_number(&value_1, &value_2) {
                return false;
            }
        } else if value_1.is_number() && value_2.is_array() {
            if value_2.len() == 0 {
                return false;
            }

            let mut json_array = JsonValue::new_array();
            let _ = json_array.push(value_1.as_u32().unwrap());

            if !compute_pair(&json_array, &value_2) {
                return false;
            }
        } else if value_1.is_array() && value_2.is_number() {
            if value_1.len() == 0 {
                return false;
            }

            let mut json_array = JsonValue::new_array();
            let _ = json_array.push(value_2.as_u32().unwrap());

            if !compute_pair(&value_1, &json_array) {
                return false;
            }
        } else {
            for i_a in 0..value_2.len() {
                if i_a as isize > value_1.len() as isize - 1 {
                    return false;
                }

                if !compute_pair(&value_1[i_a], &value_2[i_a]) {
                    return false;
                }
            }

            if value_1.len() <= value_2.len() {
                return false;
            }
        }
    }

    true
}

fn main() {
    let mut bad_order_score: u32 = 0;
    let mut pairs: Vec<[JsonValue; 2]> = Vec::new();
    let contents = fs::read_to_string("./src/sample-input.txt")
        .expect("Should have been able to read the file");

    contents.split("\n\n").for_each(|pair_lines| {
        let pair: Vec<_> = pair_lines
            .lines()
            .map(|line| json::parse(&line).unwrap())
            .collect();

        pairs.push([pair[0].clone(), pair[1].clone()]);
    });

    for (i, [pair_1, pair_2]) in pairs.iter().enumerate() {
        let result = compute_pair(pair_1, pair_2);

        println!("Compute: {}", result);
        if result {
            bad_order_score += i as u32 + 1;
            println!("i: {}", i);
        }
    }

    println!("Bad order score: {}", bad_order_score);
    // dbg!("{}", pairs);
}
