use std::fs;

const START_OFFSET_CYCLE: u32 = 20;
const STEP_CYCLE: u32 = 40;
const ADD_CYCLES: u32 = 2;

fn calcul_signal_strenght(x: i32, cycle: u32) -> i32 {
    if cycle < 20 {
        return 0;
    }

    if (cycle - START_OFFSET_CYCLE) % STEP_CYCLE == 0 { 
        println!("Signal strength: {}*{} -> {}", x, cycle, x*cycle as i32); x*(cycle as i32) } else { 0 }
}

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");
    
    /* Part One */
    let mut cycles: u32 = 0;
    let mut x: i32 = 1;
    let mut signal_strenght: i32 = 0;

    contents.lines().for_each(|line| {
        let instruction: Vec<&str> = line.split(" ").collect::<Vec<&str>>();

        match *instruction.get(0).unwrap() {
            "noop" => {cycles += 1; signal_strenght += calcul_signal_strenght(x, cycles);},
            "addx" => {
                for i in 0..ADD_CYCLES {
                    cycles += 1;
                    signal_strenght += calcul_signal_strenght(x, cycles);
                    if i == 1 {
                        x += instruction.get(1).unwrap().parse::<i32>().unwrap();
                    }
                }
            }
            _ => {},
        };
        println!("instruction: {:?}", instruction);
    });

    println!("X: {}", x);
    println!("Signal strength: {}", signal_strenght);
}
