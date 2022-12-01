use std::{fs, num};

fn main() {
    let mut number_vec: Vec<u32> = Vec::new();
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    for line in contents.split("\r\n\r\n") {
        let mut sum = 0;

        line.split("\r\n").for_each(|string_number| {
            sum += string_number.parse::<u32>().unwrap();
        });

        number_vec.push(sum);
    }

    let max = match number_vec.iter().max() {
        Some(max) => max,
        None => panic!("Vector is empty"),
    };

    println!("max: {}", max);
}
