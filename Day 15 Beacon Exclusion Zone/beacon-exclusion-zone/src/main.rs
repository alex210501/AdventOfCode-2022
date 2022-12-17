use regex::Regex;
use std::collections::HashSet;
use std::fs;

const FREQUENCY_MULTIPLICATOR: u64 = 4000000;

fn get_possible_beacon(
    map: &Vec<Sensor>,
    min_row: i32,
    max_row: i32,
    min_col: i32,
    max_col: i32,
) -> (i32, i32) {
    let beacon_in_place: HashSet<(i32, i32)> = map
        .iter()
        .filter(|sensor| {
            sensor.beacon.0 >= min_col
                && sensor.beacon.0 <= max_col
                && sensor.beacon.1 >= min_row
                && sensor.beacon.1 <= max_row
        })
        .map(|sensor| sensor.beacon)
        .collect();
    let mut r = min_row;

    while r <= max_row {
        let mut c = min_col;
        // println!("r: {}", r);
        while c <= max_col {
            let mut is_impossible_beacon = true;

            for sensor in map.iter() {
                // println!("{}-{}", c, r);
                let distance = (r - sensor.position.1).abs() + (c - sensor.position.0).abs();

                if distance > sensor.manhattan_distance() {
                    is_impossible_beacon &= true;
                } else {
                    // println!("yop");
                    is_impossible_beacon &= false;
                    // println!("beacon found: {}-{}", c, r);
                    c = sensor.position.0 + sensor.manhattan_distance() - (sensor.position.1 - r).abs();
                    // println!("new c: {}", c);
                    break;
                }
            }  
            
            if is_impossible_beacon {
                println!("{}-{}", c, r);
                return (c, r);
            }

            c += 1;
        }
        
        r += 1;
    }

    (-1, -1)
}

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
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    contents.lines().for_each(|line| {
        for cap in re.captures_iter(line) {
            map.push(Sensor {
                position: (parse_to_i32(&cap[1]), parse_to_i32(&cap[2])),
                beacon: (parse_to_i32(&cap[3]), parse_to_i32(&cap[4])),
            });
        }
    });

    let row_to_check: i32 = 2000000;
    let rows: Vec<i32> = map
        .iter()
        .flat_map(|sensor| vec![sensor.position.0, sensor.beacon.0])
        .collect();
    let cols: Vec<i32> = map
        .iter()
        .flat_map(|sensor| vec![sensor.position.1, sensor.beacon.1])
        .collect();
    let min_row = *rows.iter().min().unwrap() - 3000000;
    let max_row = *rows.iter().max().unwrap() + 3000000;
    let min_col = *cols.iter().min().unwrap() - 3000000;
    let max_col = *cols.iter().max().unwrap() + 3000000;

    /* Part One */
    let beacons_in_row: HashSet<(i32, i32)> = map
        .iter()
        .filter(|sensor| sensor.beacon.1 == row_to_check)
        .map(|sensor| sensor.beacon)
        .collect();
    let mut possibly_not_exists_sensor: u32 = 0;
    
    for c in min_col..=max_col {
        if beacons_in_row.contains(&(c, row_to_check)) {
            continue;
        }

        for sensor in map.iter() {
            let distance = (row_to_check - sensor.position.1).abs() + (c - sensor.position.0).abs();

            if distance <= sensor.manhattan_distance() {
                possibly_not_exists_sensor += 1;
                break;
            }
        }
    }

    /* Part two */
    let possible_beacon = get_possible_beacon(&map, 0, 4000000, 0, 4000000);
    let frequency_tune: u64 = possible_beacon.0 as u64 * FREQUENCY_MULTIPLICATOR + possible_beacon.1 as u64;

    println!("Possibly no exists sensor: {}", possibly_not_exists_sensor);
    println!("Possible beacon: {:?}", possible_beacon);
    println!("Frequency tune: {:?}", frequency_tune);
    // dbg!("{}",  map);
}
