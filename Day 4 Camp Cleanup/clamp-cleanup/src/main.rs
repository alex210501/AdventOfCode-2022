use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
struct Assignment {
    start: u16,
    end: u16,
}

impl Assignment {
    fn from_string(assignment: &str) -> Self {
        let values: Vec<u16> = assignment.split('-')
                                        .map(|string| string.parse::<u16>().unwrap_or(0))
                                        .collect();
        
        Self {
            start: values[0],
            end: values[1],
        }
    }

    fn is_fully_overlaped(&self, other: &Assignment) -> bool {
        (self.start <= other.start) && (self.end >= other.end)
    }

    fn is_overlaped(&self, other: &Assignment) -> bool {
        ((self.start <= other.start) && (self.end >= other.start)) ||
        ((self.start <= other.end) && (self.end >= other.end))
    }
}

fn main() {
    let file_path = "./src/input.txt";

    /* Part One */
    let reader = BufReader::new(File::open(file_path).expect(&format!("Cannot open {}", file_path)));
    let mut fully_overlap: u16 = 0;

    reader.lines().for_each(|line_result| {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => return,
        };

        let current_pair: Vec<Assignment> = line.split(',')
                                                .map(|elem| Assignment::from_string(elem))
                                                .collect();
        
        if current_pair[0].is_fully_overlaped(&current_pair[1]) || 
            current_pair[1].is_fully_overlaped(&current_pair[0]) {
                fully_overlap += 1;
        }
    });

    /* Part two */
    let reader = BufReader::new(File::open(file_path).expect(&format!("Cannot open {}", file_path)));
    let mut overlap: u16 = 0;

    reader.lines().for_each(|line_result| {
        let current_pair: Vec<Assignment> = match line_result {
            Ok(line) => line.split(',')
                            .map(|elem| Assignment::from_string(elem))
                            .collect(),
            Err(_) => return,
        };

        if current_pair[0].is_overlaped(&current_pair[1]) || current_pair[1].is_overlaped(&current_pair[0]) {
            overlap += 1;
        }
    });

    println!("Overlap part one: {}", fully_overlap);
    println!("Overlap part two: {}", overlap);
}
