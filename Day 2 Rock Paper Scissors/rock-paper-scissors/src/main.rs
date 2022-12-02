use std::{fs, iter::Enumerate};
use regex::Regex;


fn lose_against(value: &str) -> &str {
    match value {
        "A" => "Y",
        "B" => "Z",
        "C" => "X",
        "X" => "B",
        "Y" => "C",
        "Z" => "A",
        _ => "",
    }
}

fn get_alternative(value: &str) -> &str {
    match value {
        "A" => "X",
        "B" => "Y",
        "C" => "Z",
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _ => "",
    }
}

fn get_point_value(value: &str) -> u16 {
    match value {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn get_game_score(player: &str, enemy: &str) -> u16 {
    if player == get_alternative(enemy) {
        return 3;
    }

    if lose_against(player) == enemy {
        return 0;
    } 

    6
}

fn main() {
    let re = Regex::new(r"([A-Z]) ([A-Z])").unwrap();
    let mut total_score = 0;
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    contents.split("\n").for_each(|line| {
        for elem in re.captures_iter(line) {
            let enemy = &elem[1];
            let player = &elem[2];
        
            total_score += get_game_score(player, enemy) + get_point_value(player);
        }
    });

    println!("Score: {}", total_score);
}
