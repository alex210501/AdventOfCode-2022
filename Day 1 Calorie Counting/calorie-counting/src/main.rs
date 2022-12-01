use std::fs;

fn main() {
    let mut number_vec: Vec<u32> = Vec::new();
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    contents.split("\n\n").for_each(|line| {
        number_vec.push(line.split("\n").fold(0, |acc, elem| acc + elem.parse::<u32>().unwrap_or(0)));
    });

    number_vec.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let (&first, &second, &third) = {
        (number_vec.get(0).unwrap_or(&0),
        number_vec.get(1).unwrap_or(&0),
        number_vec.get(2).unwrap_or(&0),)
    };

    println!("First: {}", first);
    println!("Second: {}", second);
    println!("Third: {}", third);
    println!("Sum: {}", first + second + third);
}
