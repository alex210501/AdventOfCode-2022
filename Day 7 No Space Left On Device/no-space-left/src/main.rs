use std::fs;

const MAX_VALUE: u32 = 100000;
const CAPACITY_NEEDED: u32 = 40000000;

struct FileSystem {
    stacks: Vec<u32>,
    sum: Vec<u32>,
}

impl FileSystem {
    fn new() -> Self {
        Self {stacks: Vec::new(), sum: Vec::new() }
    }

    fn change_directory(&mut self, command: &str) {
        let [_, dir] = {
            let words: Vec<&str> = command.split(" ").collect();

            [words[0], words[1]]
        };

        if dir == "/" {
            self.stacks.push(0);
        } else if dir == ".." {
            let value = self.stacks.pop().unwrap();
            let dir_value = self.stacks.pop().unwrap();
            self.stacks.push(value + dir_value);
            self.sum.push(value);
        } else {
            self.stacks.push(0);
        }
    }

    fn list(&mut self, result: &str) {
        let base =  self.stacks.pop().unwrap();
        let value = result.lines().enumerate().fold(0, |acc, (i, line)| {
            if i == 0 {
                return acc;
            }

            let [prefix, _] = {
                let line_splitted: Vec<&str> = line.trim().split(" ").collect();

                [line_splitted[0], line_splitted[1]]
            };

            if prefix != "dir" {
                return prefix.parse::<u32>().unwrap() + acc;
            }

            acc
        });
        
        self.stacks.push(value + base);
    }

    fn get_sum_less_than(&self, max_value: u32) -> u32 {
        self.sum.iter().fold(0, |acc, &value| {
            if value <= max_value {
                return value + acc;
            }

            acc
        })
    }

    fn get_folder_capacity_remove(&self, max_capacity: u32) -> u32 {
        // The max value is the size of the entire folder
        let &folder_size = self.sum.iter().max().unwrap();
        let space_needed = folder_size - max_capacity;

        self.sum.iter().fold(u32::MAX, |acc, &value| {
            if value > space_needed && value < acc {
                return value;
            }

            acc
        })
    }

    fn create_from_commands(commands: &str) {
        let mut instance = Self::new();

        // Split by every commands
        commands.split("$").for_each(|command| {
            let command = command.trim();

            if command.starts_with("cd") { 
                instance.change_directory(command);
            } else if command.starts_with("ls") {
                instance.list(command);
            }
        });

        // Empty the stack
        while !instance.stacks.is_empty() {
            let value = instance.stacks.pop().unwrap();
            instance.sum.push(value);

            if !instance.stacks.is_empty() {
                let base = instance.stacks.pop().unwrap();
                instance.stacks.push(base + value);
            }
        }

        println!("Part one: {}", instance.get_sum_less_than(MAX_VALUE));
        println!("Part two: {}", instance.get_folder_capacity_remove(CAPACITY_NEEDED));
    }
}
fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let _ = FileSystem::create_from_commands(&contents);
}
