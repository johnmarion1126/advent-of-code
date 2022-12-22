use std::{collections::HashSet, time::Instant};

#[derive(Debug)]
struct Sensor {
    sensor_position: (i32, i32),
    beacon_position: (i32, i32),
}

impl Sensor {
    fn new(sensor_position: (i32, i32), beacon_position: (i32, i32)) -> Self {
        Self {
            sensor_position,
            beacon_position,
        }
    }

    fn manhattan_distance(&self) -> i32 {
        (self.sensor_position.0 - self.beacon_position.0).abs()
            + (self.sensor_position.1 - self.beacon_position.1).abs()
    }
}

fn parse_input(input: &str) -> Option<Vec<Sensor>> {
    let mut sensors: Vec<Sensor> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.trim().split(':').collect();

        let sensor_info = parts[0];
        let sensor_x_index = sensor_info.find("x=")?;
        let sensor_y_index = sensor_info.find(", y=")?;

        let sensor_x = sensor_info[sensor_x_index + 2..sensor_y_index]
            .parse::<i32>()
            .expect("Parsing Error");
        let sensor_y = sensor_info[sensor_y_index + 4..]
            .parse::<i32>()
            .expect("Parsing Error");

        let beacon_info = parts[1];
        let beacon_x_index = beacon_info.find("x=")?;
        let beacon_y_index = beacon_info.find(", y=")?;

        let beacon_x = beacon_info[beacon_x_index + 2..beacon_y_index]
            .parse::<i32>()
            .expect("Parsing Error");
        let beacon_y = beacon_info[beacon_y_index + 4..]
            .parse::<i32>()
            .expect("Parsing Error");

        sensors.push(Sensor::new((sensor_y, sensor_x), (beacon_y, beacon_x)));
    }

    Some(sensors)
}

fn get_sensor_area(sensors: Vec<Sensor>, target_row: i32) -> usize {
    let mut positions: HashSet<i32> = HashSet::new();

    for sensor in sensors {
        let y_distance = (target_row - sensor.sensor_position.0).abs();
        let distance = sensor.manhattan_distance();

        if target_row <= sensor.sensor_position.0 + distance
            && target_row >= sensor.sensor_position.0 - distance
        {
            let occupied_cells = (
                (sensor.sensor_position.1 - distance) + y_distance,
                (sensor.sensor_position.1 + distance) - y_distance,
            );

            for num in occupied_cells.0..=occupied_cells.1 {
                positions.insert(num);
            }

            if sensor.beacon_position.0 == target_row
            {
                positions.remove(&sensor.beacon_position.0);
            }
        }
    }

    positions.len() 
}

fn main() {
    let sensors = parse_input(
        "Sensor at x=3907621, y=2895218: closest beacon is at x=3790542, y=2949630
Sensor at x=1701067, y=3075142: closest beacon is at x=2275951, y=3717327
Sensor at x=3532369, y=884718: closest beacon is at x=2733699, y=2000000
Sensor at x=2362427, y=41763: closest beacon is at x=2999439, y=-958188
Sensor at x=398408, y=3688691: closest beacon is at x=2275951, y=3717327
Sensor at x=1727615, y=1744968: closest beacon is at x=2733699, y=2000000
Sensor at x=2778183, y=3611924: closest beacon is at x=2275951, y=3717327
Sensor at x=2452818, y=2533012: closest beacon is at x=2733699, y=2000000
Sensor at x=88162, y=2057063: closest beacon is at x=-109096, y=390805
Sensor at x=2985370, y=2315046: closest beacon is at x=2733699, y=2000000
Sensor at x=2758780, y=3000106: closest beacon is at x=3279264, y=2775610
Sensor at x=3501114, y=3193710: closest beacon is at x=3790542, y=2949630
Sensor at x=313171, y=1016326: closest beacon is at x=-109096, y=390805
Sensor at x=3997998, y=3576392: closest beacon is at x=3691556, y=3980872
Sensor at x=84142, y=102550: closest beacon is at x=-109096, y=390805
Sensor at x=3768533, y=3985372: closest beacon is at x=3691556, y=3980872
Sensor at x=2999744, y=3998031: closest beacon is at x=3691556, y=3980872
Sensor at x=3380504, y=2720962: closest beacon is at x=3279264, y=2775610
Sensor at x=3357940, y=3730208: closest beacon is at x=3691556, y=3980872
Sensor at x=1242851, y=838744: closest beacon is at x=-109096, y=390805
Sensor at x=3991401, y=2367688: closest beacon is at x=3790542, y=2949630
Sensor at x=3292286, y=2624894: closest beacon is at x=3279264, y=2775610
Sensor at x=2194423, y=3990859: closest beacon is at x=2275951, y=3717327",
    )
    .unwrap();
    let y = 2000000;

    let now = Instant::now();
    println!("Result: {}", get_sensor_area(sensors, y));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

//================================================================================================
