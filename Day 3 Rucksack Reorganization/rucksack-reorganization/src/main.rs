use std::{fs, collections::HashMap};

const STEP: usize = 3;

fn get_priority(c: char) -> u16 {
    let number_c = c as u16;

    if (('a' as u16) <= number_c) && (number_c <= ('z' as u16)) {
        return number_c - ('a' as u16) + 1;
    } else if (('A' as u16) <= number_c) && (number_c <= ('Z' as u16)) {
        return number_c - ('A' as u16) + 27;
    } else {
        return 0;
    }
}

fn main() {
    let mut sum: u32 = 0;
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    /* Part one */
    contents.split('\n').for_each(|rucksack| {
        let mid = rucksack.len() / 2;
        let mut char_map: HashMap<char, usize> = HashMap::new();
        
        for (i, c) in rucksack.chars().enumerate() {
            if i < mid {
                char_map.insert(c, i);
            } else {
                if char_map.contains_key(&c) {
                    sum += get_priority(c) as u32;
                    break;
                }
            }
        }
    });

    /* Part two */
    let lines: Vec<&str> = contents.split('\n').collect();
    let max_iterations = lines.len() / 3;
    let mut iteration = 0;
    let mut sum_part_2: u32 = 0;

    while iteration < max_iterations {
        let start_index = iteration*STEP;
        let char_map: &mut [HashMap<char, u16>; STEP] = &mut [HashMap::new(), HashMap::new(), HashMap::new()];
        
        // Insert in the HashMap of every line
        lines[start_index..start_index + STEP].iter().enumerate().for_each(|(line_nb, &line)| {
            line.chars().for_each(|c| {
                if !char_map[line_nb].contains_key(&c) {
                    char_map[line_nb].insert(c, 1);
                }
            });
        });

        // For each char, check if all hashmap has it
        // If it's the case then add the priority of the character to sum
        for (&c, _) in &char_map[0] {
            if char_map.iter().all(|map| map.contains_key(&c)) {
                sum_part_2 += get_priority(c) as u32;
            }
        }
        iteration += 1;
    }
    
    println!("Sum Part 1: {}", sum);
    println!("Sum Part 2: {}", sum_part_2);
}
