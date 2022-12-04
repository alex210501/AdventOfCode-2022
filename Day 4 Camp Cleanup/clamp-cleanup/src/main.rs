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

    fn is_overlaped(&self, other: &Assignment) -> bool {
        (self.start <= other.start) && (self.end >= other.end)
    }
}

fn main() {
    let reader = BufReader::new(File::open("./src/input.txt").expect("Cannot open sample-input.txt"));

    /* Part One */
    let mut overlap = 0;

    reader.lines().for_each(|line_result| {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => return,
        };

        let current_pair: Vec<Assignment> = line.split(',')
                                                .map(|elem| Assignment::from_string(elem))
                                                .collect();
        
        if current_pair[0].is_overlaped(&current_pair[1]) || current_pair[1].is_overlaped(&current_pair[0]) {
            overlap += 1;
        }
    });

    println!("Overlap part one: {}", overlap);
}
