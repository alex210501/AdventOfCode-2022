use std::{
    collections::HashMap,
    fs,
};
use std::rc::Rc;

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
        println!("1value: {:?}", self.stacks);

        let base =  self.stacks.pop().unwrap();
        println!("2value: {:?}", self.stacks);
        let value = result.lines().enumerate().fold(0, |acc, (i, line)| {
            if i == 0 {
                return acc;
            }

            let [prefix, _] = {
                let line_splitted: Vec<&str> = line.trim().split(" ").collect();

                [line_splitted[0], line_splitted[1]]
            };

            if prefix != "dir" {
                println!("{:?}", prefix.parse::<u32>().unwrap());

                return prefix.parse::<u32>().unwrap() + acc;
            }

            acc
        });
        println!("value: {}", value);
        self.stacks.push(value + base);
        // self.sum.push(value + base);
    }

    fn get_sum_less_than(&self) -> u32 {
        let mut sum_value = 0;
        for &value in self.sum.iter() {
            if value <= 100000 {
                sum_value += value;
            }
        }

        dbg!("{}", sum_value);
        sum_value
    }

    fn create_from_commands(commands: &str) {
        let mut instance = Self::new();

        // Split by every commands
        commands.split("$").for_each(|command| {
            let command = command.trim();

            println!("single_words: {:?}", command);
            if command.starts_with("cd") { 
                instance.change_directory(command);
            } else if command.starts_with("ls") {
                instance.list(command);
            }

            dbg!("{}", &instance.stacks);        
            dbg!("{}", &instance.sum);
        });

        dbg!("{:?}", &instance.stacks);        
        dbg!("{:?}", &instance.sum);
        instance.get_sum_less_than();
    }
}
fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let _ = FileSystem::create_from_commands(&contents);
}
