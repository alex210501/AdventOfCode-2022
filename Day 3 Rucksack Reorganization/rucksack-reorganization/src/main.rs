use std::{fs, collections::HashMap};

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

    println!("Sum: {}", sum);
}
