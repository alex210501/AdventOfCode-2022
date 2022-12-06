use std::{fs, collections::HashMap};

const STEP: usize = 4;

fn get_marker(text: &str) -> i32 {
    let mut lower: usize = 0;

    // Iterate over each character
    for (i, _) in text.char_indices() {
        if (i - lower) < STEP {
            continue;
        }

        // Create HashMap for the current string window
        let mut char_map: HashMap<char, u32> = HashMap::new();

        // Add character count in the HashMap
        text[lower..i].chars().for_each(|slice_c| {
            match char_map.get(&slice_c) {
                Some(value) => char_map.insert(slice_c, value + 1),
                None => char_map.insert(slice_c, 1),
            };
        });

        // Check that all characters appaers one time
        if char_map.values().all(|&value| value < 2) {
            return i as i32;
        }

        lower += 1;
    }

    -1
}

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");
    
    println!("Marker Part One: {}", get_marker(&contents));
}
