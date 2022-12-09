use std::{fs, collections::HashSet};

fn head_touch_tail(head: (i32, i32), tail: (i32, i32)) -> bool {
    // (head.0 + 1 == tail.1 && head.1 == tail.1) || (head.0 - 1 == tail.1 && head.1 == tail.1) ||
    //     (head.1 + 1 == tail.1 && head.0 == tail.0) || (head.1 - 1 == tail.1 && head.0 == tail.0)
    (head.0 <= tail.0 + 1) && (head.0 >= tail.0 - 1) && (head.1 <= tail.1 + 1) && (head.1 >= tail.1 - 1)
}

fn main() {
    let mut tail: (i32, i32) = (0, 0);
    let mut head: (i32, i32) = (0, 0);
    let mut position_visited: HashSet<(i32, i32)> = HashSet::new();

    /* Part One */
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    contents.lines().for_each(|line| {
        // Match the line
        let [direction, steps] = {
            let tmp: Vec<&str> = line.trim().split(" ").collect();

            [tmp[0], tmp[1]]
        };

        println!("direction: {}", direction);

        for _ in 0..steps.parse::<i32>().unwrap() {

            println!("tail:  {:?}", tail);
            println!("head:  {:?}", head);

            match direction {
                "R" => {head = (head.0, head.1 + 1); /*tail = (head.0, head.1 - 1);*/},
                "L" => {head = (head.0, head.1 - 1); /*tail = (head.0, head.1 + 1);*/},
                "U" => {head = (head.0 + 1, head.1); /*tail = (head.0 - 1, head.1 );*/},
                "D" => {head = (head.0 - 1, head.1); /*tail = (head.0 + 1, head.1);*/},
                _ => {}
            }

            // Check if the tail is touching the head
            if !head_touch_tail(head, tail) {
                match direction {
                    "R" => tail = (head.0, head.1 - 1),
                    "L" => tail = (head.0, head.1 + 1),
                    "U" => tail = (head.0 - 1, head.1 ),
                    "D" => tail = (head.0 + 1, head.1),
                    _ => {}
                }
            }

            position_visited.insert(tail);
        }
    });

    println!("Position visited by tails: {}", position_visited.len());
}
