use std::fs;

use json::JsonValue;
extern crate json;
// use json;

#[derive(Debug, PartialEq, Eq)]
enum Results {
    CORRECT,
    WRONG,
    CONTINUE,
}

fn compute_mix(pair_1: &JsonValue, pair_2: &JsonValue) -> Results {
    if pair_2.is_null() && !pair_1.is_null() {
        return Results::WRONG;
    }

    if pair_1.is_null() && pair_2.is_null() {
        return Results::CONTINUE;
    }

    if pair_1.is_null() {
        return Results::CORRECT;
    }

    if pair_1.is_array() && pair_2.is_number() {
        let mut array = JsonValue::new_array();
        let _ = array.push(pair_2.as_u32().unwrap());

        return compute_array(pair_1, &array);
    } else if pair_1.is_number() && pair_2.is_array(){
        let mut array = JsonValue::new_array();
        let _ = array.push(pair_1.as_u32().unwrap());

        return compute_array(&array, pair_2);
    } else {
        return Results::CONTINUE;
    }
}

fn compute_number(pair_1: &JsonValue, pair_2: &JsonValue) -> Results {
    let number_1 = pair_1.as_u32().unwrap();
    let number_2 = pair_2.as_u32().unwrap();

    if number_1 < number_2 {
        return Results::CORRECT;
    } else if number_1 > number_2 {
        return Results::WRONG;
    } else {
        return Results::CONTINUE;
    }
}

fn compute_array(pair_1: &JsonValue, pair_2: &JsonValue) -> Results {
    let loops = if pair_1.len() < pair_2.len() {pair_2.len()} else {pair_1.len()};

    for i in 0..loops {
        let value_1 = &pair_1[i];
        let value_2 = &pair_2[i];

        if value_1.is_null() && value_2.is_null() {
            return Results::CONTINUE;
        }

        if value_1.is_null() {
            return Results::CORRECT;
        }

        if value_2.is_null() {
            return Results::WRONG;
        }

        let result = compute_pair(&value_1, &value_2);
        if result != Results::CONTINUE {
            return result;
        }
    }

    Results::CONTINUE
}

fn compute_pair(pair_1: &JsonValue, pair_2: &JsonValue) -> Results {
    if pair_1.is_number() && pair_2.is_number() {
        return compute_number(&pair_1, &pair_2);
    } else if pair_1.is_array() && pair_2.is_array() {
        return compute_array(&pair_1, &pair_2);
    } else {
        return compute_mix(pair_1, pair_2);
    }
}

fn main() {
    let mut good_order_score: u32 = 0;
    let mut pairs: Vec<[JsonValue; 2]> = Vec::new();
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    contents.split("\n\n").for_each(|pair_lines| {
        let pair: Vec<_> = pair_lines
            .lines()
            .map(|line| json::parse(&line).unwrap())
            .collect();

        pairs.push([pair[0].clone(), pair[1].clone()]);
    });

    /* Part One */
    for (i, [pair_1, pair_2]) in pairs.iter().enumerate() {
        let result = compute_pair(pair_1, pair_2);
        match result {
            Results::CORRECT => {
                good_order_score += i as u32 + 1;
            }
            _ => {}
        };
    }

    /* Part two */
    let first_divider = "[[2]]";
    let second_divider = "[[6]]";
    let mut line_vec: Vec<JsonValue> = vec![json::parse(first_divider).unwrap(), json::parse(second_divider).unwrap()];
    pairs.iter().for_each(|x| line_vec.extend_from_slice(x));

    for i in 0..line_vec.len() {
        for j in i..line_vec.len() {
            let val_1 = &line_vec[i];
            let val_2 = &line_vec[j];
            match compute_pair(&val_1, &val_2) {
                Results::CORRECT => {},
                _ => {line_vec.swap(i, j)},
            };
        }
    }

    let first_divider_position = line_vec.iter().position(|x| x.to_string() == first_divider).unwrap() + 1;
    let seconds_divider_position = line_vec.iter().position(|x| x.to_string() == second_divider).unwrap() + 1;

    println!("Good order score: {}", good_order_score);
    println!("First divider: {}", first_divider_position);
    println!("Second divider: {}", seconds_divider_position);
    println!("Part two: {}", first_divider_position*seconds_divider_position);
}
