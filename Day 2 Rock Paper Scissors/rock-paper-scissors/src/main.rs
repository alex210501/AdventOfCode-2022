use std::fs;
use regex::Regex;

fn win_against(value: &str) -> &str {
    match value {
        "Y" => "A",
        "Z" => "B",
        "X" => "C",
        "B" => "X",
        "C" => "Y",
        "A" => "Z",
        _ => "",
    }
}

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
    // Draw
    if player == get_alternative(enemy) {
        return 3;
    }

    // Lose
    if lose_against(player) == enemy {
        return 0;
    } 

    // Win
    6
}

fn get_player_to_play<'a>(enemy: &'a str, expected_result: &'a str) -> &'a str{
    match expected_result {
        "X" => win_against(enemy),
        "Y" => get_alternative(enemy),
        "Z" => lose_against(enemy),
        _ => "",
    }
}

fn main() {
    // Regex just for fun
    let re = Regex::new(r"([A-Z]) ([A-Z])").unwrap();
    let mut total_score = 0;
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    /* Part One */
    contents.lines().for_each(|line| {
        for elem in re.captures_iter(line) {
            let enemy = &elem[1];
            let player = &elem[2];
        
            total_score += get_game_score(player, enemy) + get_point_value(player);
        }
    });

    /* Part Two */
    let mut total_score_part_2 = 0;

    contents.lines().for_each(|line| {
        for elem in re.captures_iter(line) {
            let enemy = &elem[1];
            let expected_result = &elem[2];
            let player = get_player_to_play(enemy, expected_result);
        
            total_score_part_2 += get_game_score(player, enemy) + get_point_value(player);
        }
    });

    println!("Score: {}", total_score);
    println!("Score part 2: {}", total_score_part_2);
}
