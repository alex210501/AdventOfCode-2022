use std::fs;
use regex::Regex;

const SIZE_VALUE: usize = 3;

fn get_max_from_tours(tour: &str) -> u32 {
    tour.lines().last().map(|nb_line| {
        nb_line.split("   ").map(|elem| elem.trim().parse::<u32>().unwrap_or(0))
    }).unwrap().max().unwrap_or(0)
}

fn get_top_values(stacks: &Vec<Vec<&str>>) -> String {
    stacks.iter().fold("".to_string(), |acc, stack| {
        format!("{}{}", acc, &stack.last().unwrap_or(&"")[1..2])
    })
}

fn part_one(moves: &str, stacks: &mut Vec<Vec<&str>>) -> String {
    let re: Regex = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    moves.lines().for_each(|line| {
        for elem in re.captures_iter(line) {
            let mut nbr_to_move = elem[1].parse::<u16>().unwrap_or(0);
            let source = elem[2].parse::<usize>().unwrap_or(0) - 1;
            let dest = elem[3].parse::<usize>().unwrap_or(0) - 1;
        
            while nbr_to_move > 0 {
                let value: &str = stacks[source].pop().unwrap();
                stacks[dest].push(value);
                nbr_to_move -= 1;
            }
        }
    });

    get_top_values(stacks)
}

fn part_two(moves: &str, stacks: &mut Vec<Vec<&str>>) -> String {
    let re: Regex = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    moves.lines().for_each(|line| {
        for elem in re.captures_iter(line) {
            let mut nbr_to_move = elem[1].parse::<u16>().unwrap_or(0);
            let source = elem[2].parse::<usize>().unwrap_or(0) - 1;
            let dest = elem[3].parse::<usize>().unwrap_or(0) - 1;
            let mut values_to_move: Vec<&str> = Vec::new();
        
            while nbr_to_move > 0 {
                values_to_move.push(stacks[source].pop().unwrap_or(""));
                nbr_to_move -= 1;
            }

            values_to_move.iter().rev().for_each(|&value| stacks[dest].push(value));
        }
    });
    
    get_top_values(stacks)
}

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    // Get the tour and the moves
    let (tour, moves) = {
        let split_content: Vec<&str> = contents.split("\n\n").collect();

        (split_content[0], split_content[1])
    };

    // Get the max element for construct the stacks with specific size
    let max_element: u32 = get_max_from_tours(&tour);
    let mut stacks: Vec<Vec<&str>> = vec![Vec::new(); max_element as usize];
    let mut stacks_part_two: Vec<Vec<&str>> = vec![Vec::new(); max_element as usize];

    // Construct the tours using stacks
    tour.lines().rev().enumerate().for_each(|(i, line)| {
        if i == 0 {
            return;
        }

        (0..line.len()).step_by(SIZE_VALUE).for_each(|index| {
            let index_shifted = index + index/SIZE_VALUE;

            if index_shifted < line.len() {
                let string_to_insert = line[index_shifted..index_shifted + SIZE_VALUE].trim();

                if string_to_insert != "" {
                    stacks[index/SIZE_VALUE].push(string_to_insert);
                    stacks_part_two[index/SIZE_VALUE].push(string_to_insert);
                }
            }
        });
    });

    println!("Values for part one: {}", part_one(&moves, &mut stacks));
    println!("Values for part two: {}", part_two(&moves, &mut stacks_part_two));
}
