use std::fs;
use regex::Regex;

const SIZE_VALUE: usize = 3;

fn main() {
    let re: Regex = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    /* Part One */
    let (tour, moves) = {
        let split_content: Vec<&str> = contents.split("\n\n").collect();

        (split_content[0], split_content[1])
    };

    let max_element: u16 = tour.lines().last().map(|nb_line| {
        nb_line.split("   ").map(|elem| elem.trim().parse::<u16>().unwrap_or(0))
    }).unwrap().max().unwrap_or(0);
    let mut stacks: Vec<Vec<&str>> = vec![Vec::new(); max_element as usize];

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
                }
            }
        });
    });

    // Set moves
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

    let top_values = stacks.iter().fold("".to_string(), |acc, stack| {
        format!("{}{}", acc, &stack.last().unwrap_or(&"")[1..2])
    });

    println!("{:?}", stacks);
    println!("Values for part one: {}", top_values);
}
