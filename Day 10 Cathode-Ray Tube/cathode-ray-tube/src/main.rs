use std::fs;

const START_OFFSET_CYCLE: u32 = 20;
const STEP_CYCLE: u32 = 40;
const ADD_CYCLES: u32 = 2;
const SPRITE_WIDE: u32 = 3;
const CRT_ROWS: usize = 6;
const CRT_COLS: usize = 40;

fn calcul_signal_strenght(x: i32, cycle: u32) -> i32 {
    if cycle < 20 {
        return 0;
    }

    if (cycle - START_OFFSET_CYCLE) % STEP_CYCLE == 0 {
        x * (cycle as i32)
    } else {
        0
    }
}

fn add_pixels(rows: &mut [[char; CRT_COLS]; CRT_ROWS], x: i32, cycle: i32) {
    let row: usize = cycle as usize / CRT_COLS;
    let col: usize = cycle as usize % CRT_COLS;
    let crt: i32 = cycle % CRT_COLS as i32;
    let sprite_exteriour: i32 = SPRITE_WIDE as i32 / 2;

    if x - sprite_exteriour <= crt && crt <= x + sprite_exteriour {
        rows[row][col] = '#';
    } else {
        rows[row][col] = '.';
    }
}

fn main() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    /* Part One */
    let mut cycles: u32 = 0;
    let mut x: i32 = 1;
    let mut signal_strenght: i32 = 0;

    contents.lines().for_each(|line| {
        let instruction: Vec<&str> = line.split(" ").collect::<Vec<&str>>();

        match *instruction.get(0).unwrap() {
            "noop" => {
                cycles += 1;
                signal_strenght += calcul_signal_strenght(x, cycles);
            }
            "addx" => {
                for i in 0..ADD_CYCLES {
                    cycles += 1;
                    signal_strenght += calcul_signal_strenght(x, cycles);
                    if i == 1 {
                        x += instruction.get(1).unwrap().parse::<i32>().unwrap();
                    }
                }
            }
            _ => {}
        };
    });

    /* Part Two */
    let mut cycles: u32 = 0;
    let mut x: i32 = 1;
    let mut rows: [[char; CRT_COLS]; CRT_ROWS] = [['\0'; CRT_COLS]; CRT_ROWS];

    contents.lines().for_each(|line| {
        let instruction: Vec<&str> = line.split(" ").collect::<Vec<&str>>();

        match *instruction.get(0).unwrap() {
            "noop" => {
                add_pixels(&mut rows, x, cycles as i32);
                cycles += 1;
            }
            "addx" => {
                for i in 0..ADD_CYCLES {
                    add_pixels(&mut rows, x, cycles as i32);
                    cycles += 1;
                    if i == 1 {
                        x += instruction.get(1).unwrap().parse::<i32>().unwrap();
                    }
                }
            }
            _ => {}
        };
    });

    println!("X: {}", x);
    println!("Signal strength: {}", signal_strenght);
    rows.iter().map(|row| row.iter().collect::<String>()).for_each(|row| println!("{}", row));
}
