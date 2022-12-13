use std::{collections::HashSet, fs, hash::Hash};

extern crate queues;
use queues::*;

const START_CHAR: char = 'S';
const END_CHAR: char = 'E';

#[derive(Clone, Copy)]
struct Node {
    coords: (usize, usize),
    steps: u32,
}


fn get_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (i_r, row) in grid.iter().enumerate() {
        for (i_c, &col) in row.iter().enumerate() {
            if col == START_CHAR {
                return (i_r, i_c);
            }
        }
    }

    (usize::MAX, usize::MAX)
}

fn get_end(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (i_r, row) in grid.iter().enumerate() {
        for (i_c, &col) in row.iter().enumerate() {
            if col == END_CHAR {
                return (i_r, i_c);
            }
        }
    }

    (usize::MAX, usize::MAX)
}

fn get_neightboors(grid: &Vec<Vec<char>>, node: Node) -> Vec<Node> {
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();
    let (y, x) = node.coords;
    let mut neighboors: Vec<Node> = Vec::new();

    if x > 0 {
        neighboors.push(Node {coords: (y, x - 1), steps: node.steps + 1});
    }

    if y > 0 {
        neighboors.push(Node {coords: (y - 1, x), steps: node.steps + 1});
    }

    if x < cols - 1 {
        neighboors.push(Node {coords: (y, x + 1), steps: node.steps + 1});
    }

    if y < rows - 1 {
        neighboors.push(Node {coords: (y + 1, x), steps: node.steps + 1});
    }

    neighboors
}

fn step_to_end(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut iterations: u32 = 0;
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut neightboors: Queue<Node> = Queue::new();
    let mut nodes: HashSet<(usize, usize)> = HashSet::new();

    let _ = neightboors.add(Node { coords: start, steps: 0 });

    while neightboors.size() > 0 {
        let node = neightboors.remove().unwrap();

        if node.coords == end {
            println!("nope");
            return node.steps;
        }

        get_neightboors(&grid, node).iter().filter(|v| {
            grid[v.coords.0][v.coords.1] as u8 <= grid[node.coords.0][node.coords.1] as u8 ||
            grid[v.coords.0][v.coords.1] as u8 == grid[node.coords.0][node.coords.1] as u8 + 1
        }).for_each(|v| {
            if !visited_nodes.contains(&v.coords) {
                visited_nodes.insert(v.coords);
                let _ = neightboors.add(v.clone());
            } 
        });
        
        // if last_len == visited_nodes.len() {
            // println!("node {:?}", neightboors.iter().map(|(y, x)| grid[*y][*x]).collect::<Vec<char>>());
        //     return 0;
        // }
    }

    0
    // return helper(&grid, end, set.clone(), &mut HashSet::new(), 0);
}

fn main() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    let mut grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let start: (usize, usize) = get_start(&grid);
    let end: (usize, usize) = get_end(&grid);

    println!("start: {}", grid[start.0][start.1]);
    println!("end: {}", grid[end.0][end.1]);

    grid[start.0][start.1] = 'a';
    grid[end.0][end.1] = 'z';

    // println!("{} {}", grid[15][108], grid[18][19]);

    println!("step_to_end: {}", step_to_end(&grid, start, end));
}
