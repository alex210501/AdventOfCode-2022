use regex::Regex;
use std::fs;
use std::collections::HashSet;

fn parse_to_i32(number: &str) -> i32 {
    number.parse::<i32>().unwrap_or(0)
}

#[derive(Debug)]
struct Sensor {
    position: (i32, i32),
    beacon: (i32, i32),
}

impl Sensor {
    fn manhattan_distance(&self) -> i32 {
        (self.position.0 - self.beacon.0).abs() + (self.position.1 - self.beacon.1).abs()
    }
}

fn main() {
    let mut map: Vec<Sensor> = vec![];
    let re = Regex::new(
        r"Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)",
    )
    .unwrap();
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    contents.lines().for_each(|line| {
        for cap in re.captures_iter(line) {
            map.push(Sensor {
                position: (parse_to_i32(&cap[1]), parse_to_i32(&cap[2])), 
                beacon: (parse_to_i32(&cap[3]), parse_to_i32(&cap[4]))
            });
        }
    });

    let row_to_check: i32 = 2000000;
    let rows: Vec<i32> = map.iter().flat_map(|sensor| vec![sensor.position.0, sensor.beacon.0]).collect();
    let cols: Vec<i32> = map.iter().flat_map(|sensor| vec![sensor.position.1, sensor.beacon.1]).collect();
    let min_row = *rows.iter().min().unwrap() - 3000000;
    let max_row = *rows.iter().max().unwrap() + 3000000;
    let min_col = *cols.iter().min().unwrap() - 3000000;
    let max_col = *cols.iter().max().unwrap() + 3000000;

    /* Part One */
    let beacons_in_row: HashSet<(i32, i32)> = map.iter().filter(|sensor| sensor.beacon.1 == row_to_check).map(|sensor| sensor.beacon).collect();
    let mut possibly_not_exists_sensor: u32 = 0;
    println!("beacons: {:?}", beacons_in_row);
    for c in min_col..=max_col {
        if beacons_in_row.contains(&(c, row_to_check)) {
            continue;
        }

        for sensor in map.iter() {
            let distance = (row_to_check - sensor.position.1).abs() + (c - sensor.position.0).abs();


            if distance <= sensor.manhattan_distance() {
                // println!("{}-{}", c, row_to_check);
                // println!("dist : {} manthan distance: {}-{} -> {}", distance, sensor.position.0, sensor.position.1, sensor.manhattan_distance());

                possibly_not_exists_sensor += 1;
                break;
            }
        }
    }

    println!("Possibly no exists sensor: {}", possibly_not_exists_sensor);
    // dbg!("{}",  map);
}
