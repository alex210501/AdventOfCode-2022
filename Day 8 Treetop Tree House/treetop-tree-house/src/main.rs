use std::{collections::HashSet, fs};

fn main() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    let mut tree_visited: HashSet<[u32; 2]> = HashSet::new();
    let mut numbers: Vec<Vec<u32>> = Vec::new();

    // Parse line
    for line in contents.lines() {
        numbers.push(Vec::new());
        for c in line.chars() {
            numbers.last_mut().unwrap().push(c.to_digit(10).unwrap());
        }
    }

    let rows: u32 = numbers.len() as u32;
    let cols: u32 = numbers[0].len() as u32;

    for i_row in (0..rows) {
        for i_col in 0..cols {
            if i_row == 0 || i_row == rows - 1 || i_col == 0 || i_col == cols - 1 {
                tree_visited.insert([i_row, i_col]);
            }

            let val = *numbers
                .get(i_row as usize)
                .unwrap()
                .get(i_col as usize)
                .unwrap();

            for index in 0..cols {
                let current_val = *numbers
                    .get(i_row as usize)
                    .unwrap()
                    .get(index as usize)
                    .unwrap();

                if index == i_col {
                    tree_visited.insert([i_row, i_col]);
                    break;
                }

                if val <= current_val {
                    break;
                }
            }
            for index in (0..cols).rev() {
                let current_val = *numbers
                    .get(i_row as usize)
                    .unwrap()
                    .get(index as usize)
                    .unwrap();

                if index == i_col {
                    tree_visited.insert([i_row, i_col]);
                    break;
                }

                if val <= current_val {
                    break;
                }
            }

            for index in (0..rows) {
                let current_val = *numbers
                    .get(index as usize)
                    .unwrap()
                    .get(i_col as usize)
                    .unwrap();

                if index == i_row {
                    tree_visited.insert([i_row, i_col]);
                    break;
                }

                if val <= current_val {
                    break;
                }
            }

            for index in (0..rows).rev() {
                let current_val = *numbers
                    .get(index as usize)
                    .unwrap()
                    .get(i_col as usize)
                    .unwrap();

                if index == i_row {
                    tree_visited.insert([i_row, i_col]);
                    break;
                }

                if val <= current_val {
                    break;
                }
            }
        }
    }

    /* Part two */
    let mut highet_scenic_score: u32 = 0;

    for [r, c] in tree_visited.iter() {
        if *r == 0 || *r == rows - 1 || *c == 0 || *c == cols - 1 {
            continue;
        }

        let mut scenic_score: u32 = 1;
        let val = *numbers.get(*r as usize).unwrap().get(*c as usize).unwrap();

        // Left
        for index in (0..*c).rev() {
            let current_val = *numbers
                .get(*r as usize)
                .unwrap()
                .get(index as usize)
                .unwrap();

            if index == 0 {
                scenic_score *= *c;
                break;
            }

            if val <= current_val {
                scenic_score *= *c - index;
                break;
            }
        }

        for index in *c + 1..cols {
            let current_val = *numbers
                .get(*r as usize)
                .unwrap()
                .get(index as usize)
                .unwrap();

            if index == cols - 1 {
                scenic_score *= cols - *c - 1;
                break;
            }

            if val <= current_val {
                scenic_score *= index - *c;
                break;
            }
        }

        for index in (0..*r).rev() {
            let current_val = *numbers
                .get(index as usize)
                .unwrap()
                .get(*c as usize)
                .unwrap();

            if index == 0 {
                scenic_score *= *r;
                break;
            }

            if val <= current_val {
                scenic_score *= *r - index;
                break;
            }
        }

        for index in *r + 1..rows {
            let current_val = *numbers
                .get(index as usize)
                .unwrap()
                .get(*c as usize)
                .unwrap();

            if index == rows - 1 {
                scenic_score *= rows - *r - 1;
                break;
            }

            if val <= current_val {
                scenic_score *= index - *r;
                break;
            }
        }

        if scenic_score > highet_scenic_score {
            highet_scenic_score = scenic_score;
        }
    }

    println!("Part One: {}", tree_visited.len());
    println!("Part Two: {}", highet_scenic_score);
}
