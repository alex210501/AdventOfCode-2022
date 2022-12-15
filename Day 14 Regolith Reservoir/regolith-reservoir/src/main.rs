use std::fs;

const LENGTH_ARRAY: usize = 1000;
const START_INDEX: (usize, usize) = (500, 0);

fn is_endless(map: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    for i in r..map.len() {
        if map[i][c] != '.' {
            return false;
        }
    }

    true
}

fn is_blocked(map: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    map[r + 1][c] != '.' && map[r + 1][c + 1] != '.' && map[r + 1][c - 1] != '.'
}

fn can_go_left(map: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    map[r + 1][c - 1] == '.'
}

fn can_go_right(map: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    map[r + 1][c + 1] == '.'
}

fn can_go_down(map: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    map[r + 1][c] == '.'
}

fn compute_sand(map: &mut Vec<Vec<char>>, r: usize, c: usize) -> bool {
    // if is_endless(map, r, c) {
    //     return false;
    // }

    if can_go_down(map, r, c) {
        return compute_sand(map, r + 1, c);
    } else if can_go_left(map, r, c) {
        return compute_sand(map, r + 1, c - 1);
    } else if can_go_right(map, r, c) {
        return compute_sand(map, r + 1, c + 1);
    } else {
        map[r][c] = 'o';
        return (c, r) != START_INDEX;
    }
}

fn main() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    let mut map = vec![vec!['.'; LENGTH_ARRAY]; LENGTH_ARRAY];

    contents
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let indices_vec: Vec<(usize, usize)> = line
                .split("->")
                .map(|indices| {
                    let tmp: Vec<usize> = indices
                        .trim()
                        .split(',')
                        .map(|nbr| nbr.parse::<usize>().unwrap())
                        .collect();
                    (tmp[1], tmp[0])
                })
                .collect();

            println!("{:?}", indices_vec);
            for i in 0..indices_vec.len() - 1 {
                if indices_vec[i].0 == indices_vec[i + 1].0 {
                    let range = if indices_vec[i].1 < indices_vec[i + 1].1 {
                        indices_vec[i].1..indices_vec[i + 1].1 + 1
                    } else {
                        indices_vec[i + 1].1..indices_vec[i].1 + 1
                    };
                    for c in range {
                        map[indices_vec[i].0][c] = '#';
                    }
                } else {
                    let range = if indices_vec[i].0 < indices_vec[i + 1].0 {
                        indices_vec[i].0..indices_vec[i + 1].0 + 1
                    } else {
                        indices_vec[i + 1].0..indices_vec[i].0 + 1
                    };
                    for r in range {
                        map[r][indices_vec[i].1] = '#';
                    }
                }
            }
        });

    let mut highest_y: usize = 0;

    for r in 0..LENGTH_ARRAY {
        for c in 0..LENGTH_ARRAY {
            if map[r][c] == '#' && r > highest_y {
                highest_y = r;
            }
        }
    }

    // Add floor
    for i in 0..LENGTH_ARRAY {
        map[highest_y + 2][i] = '#';
    }

    /* Part One */
    let mut sand_at_rest: u32 = 0;

    loop {
        if compute_sand(&mut map, START_INDEX.1, START_INDEX.0) {
            sand_at_rest += 1;
        } else {
            break;
        }
    }

    let mut test: u32 = 0;

    for r in 0..highest_y + 2 {
        for c in 200..800 {
            print!("{}", map[r][c]);
            if map[r][c] == 'o' {
                test += 1
            }
        }
        println!();
    }

    println!("{}", sand_at_rest + 1);
    println!("{}", test);
}
