use std::collections::HashSet;

use colored::*;

#[derive(Debug)]
struct Sensor {
    sensor_position: (i32, i32),
    beacon_position: (i32, i32),
}

impl Sensor {
    fn new(sensor_position: (i32, i32), beacon_position: (i32, i32)) -> Self {
        Self {
            sensor_position,
            beacon_position
        }
    }

    fn manhattan_distance(&self) -> i32 {
        (self.sensor_position.0 - self.beacon_position.0).abs() + (self.sensor_position.1 - self.beacon_position.1)
    }
}

const SIZE: usize = 20;

fn parse_input(input: &str) -> Option<Vec<Sensor>>{
    let mut sensors: Vec<Sensor> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.trim().split(':').collect();

        let sensor_info = parts[0];
        let sensor_x_index = sensor_info.find("x=")?;
        let sensor_y_index = sensor_info.find(", y=")?;

        let sensor_x = sensor_info[sensor_x_index+2..sensor_y_index].parse::<i32>().expect("Parsing Error");
        let sensor_y = sensor_info[sensor_y_index+4..].parse::<i32>().expect("Parsing Error");

        let beacon_info = parts[1];
        let beacon_x_index = beacon_info.find("x=")?;
        let beacon_y_index = beacon_info.find(", y=")?;

        let beacon_x = beacon_info[beacon_x_index+2..beacon_y_index].parse::<i32>().expect("Parsing Error");
        let beacon_y = beacon_info[beacon_y_index+4..].parse::<i32>().expect("Parsing Error");

        sensors.push(Sensor::new(
            (sensor_x, sensor_y),
            (beacon_x, beacon_y)
        ));
    }

    Some(sensors)
}

fn print_matrix(matrix: &[[char; SIZE]; SIZE], target_row: i32) {
    let mut picture: Vec<Vec<ColoredString>> = Vec::new();

    for index in 0..matrix.len() {
        let mut colored_values: Vec<ColoredString> = Vec::new();

        for character in matrix[index] {
            match character {
                'S' => colored_values.push(character.to_string().truecolor(64, 145, 108)),
                '#' => colored_values.push(character.to_string().truecolor(82, 183, 136)),
                'B' => colored_values.push(character.to_string().truecolor(45, 106, 79)),
                _ => {
                    if index == target_row as usize {
                        colored_values.push(character.to_string().truecolor(162, 210, 255));
                    } else {
                        colored_values.push(character.to_string().white());
                    }
                }
            }
        }

        picture.push(colored_values);
    }

    for row in picture {
        for value in row {
            print!("{} ", value);
        }
        println!();
    }
}

fn get_sensor_area(sensors: Vec<Sensor>, target_row: i32) -> usize {
    let mut matrix = [['x'; SIZE]; SIZE];
    let mut positions: HashSet<i32> = HashSet::new();
    let mut num_beacons_in_row = 0;

    for sensor in sensors {
        matrix[sensor.sensor_position.0 as usize][sensor.sensor_position.1 as usize] = 'S';
        matrix[sensor.beacon_position.0 as usize][sensor.beacon_position.1 as usize] = 'B';

        let distance = sensor.manhattan_distance();

        // TOP
        matrix[(sensor.sensor_position.0 - distance) as usize][sensor.sensor_position.1 as usize] = '#';
        // BOTTOM
        matrix[(sensor.sensor_position.0 + distance) as usize][sensor.sensor_position.1 as usize] = '#';

        // LEFT
        matrix[sensor.sensor_position.0 as usize][(sensor.sensor_position.1 - distance) as usize] = '#';
        // RIGHT
        matrix[sensor.sensor_position.0 as usize][(sensor.sensor_position.1 + distance) as usize] = '#';

        let diameter = 1 + (distance * 2);
        let y_distance = (target_row - sensor.sensor_position.0).abs();

        let mut occupied_cells = (0, 0);

        if target_row <= sensor.sensor_position.0 + distance 
        && target_row >= sensor.sensor_position.0 - distance {
            occupied_cells = (
                (sensor.sensor_position.1 - distance) + y_distance,
                (sensor.sensor_position.1 + distance) - y_distance,
            );

            for num in occupied_cells.0..=occupied_cells.1 {
                positions.insert(num);
            }

            if sensor.beacon_position.0 == target_row {
                num_beacons_in_row += 1;
            }
        }


        print_matrix(&matrix, target_row);
        println!("Manhanttan Distance: {}", distance);
        println!("Sensor Diameter: {}", diameter);
        println!("Occupied Cells: {:?}", occupied_cells);

        println!();
    }

    positions.len() - num_beacons_in_row
}

fn main() {
    let mut sensors: Vec<Sensor> = Vec::new();
    sensors.push(Sensor::new((4, 4), (5, 1)));
    sensors.push(Sensor::new((8, 6), (7, 5)));
    sensors.push(Sensor::new((14, 3), (13, 2)));
    sensors.push(Sensor::new((6, 14), (8, 11)));
    println!("Result: {}", get_sensor_area(sensors, 7));
}
