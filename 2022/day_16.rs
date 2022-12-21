use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    time::Instant,
};

#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: i32,
    valves: Vec<String>,
}

impl Valve {
    fn new(id: String, flow_rate: i32, valves: Vec<String>) -> Self {
        Self {
            id,
            flow_rate,
            valves,
        }
    }
}

fn parse_input(input: &str) -> (HashMap<String, Valve>, HashMap<String, usize>) {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    let mut valves_map: HashMap<String, usize> = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let id = &line.trim()[6..8];

        let equal = line.trim().find('=').unwrap();
        let semicolon = line.trim().find(';').unwrap();
        let flow_rate = line.trim()[equal + 1..semicolon].to_owned();

        let mut ids: Vec<String> = Vec::new();

        let ids_index = line.trim().find("valve").unwrap();
        let split_parts: String = line.trim()[ids_index..].to_owned().split(',').collect();
        for parts in split_parts.split(' ').skip(1) {
            ids.push(parts.trim().to_string());
        }

        let valve = Valve::new(id.to_string(), flow_rate.parse::<i32>().unwrap(), ids);

        valves_map.insert(id.to_string(), valves.len());
        valves.insert(id.to_string(), valve);
    }

    (valves, valves_map)
}

fn print_matrix(matrix: &[Vec<i16>], valves_map: &HashMap<String, usize>) {
    println!();
    print!("    ");
    let mut labels: Vec<String> = vec![String::from("00"); valves_map.len()];
    for (key, value) in valves_map.iter() {
        labels[*value] = key.to_string();
    }

    for label in labels.iter() {
        print!("{0: <7}", label);
    }

    println!();

    for index in 0..valves_map.len() {
        print!("{}  ", labels[index]);
        for item in &matrix[index] {
            print!("{0: <7}", item);
        }

        println!();
    }

    println!();
}

fn floyd_warshall_algorithm(
    valves: &HashMap<String, Valve>,
    valves_map: &HashMap<String, usize>,
) -> (Vec<Vec<i16>>, HashMap<usize, String>) {
    let num_of_vertices = valves_map.len();
    let mut matrix: Vec<Vec<i16>> = vec![vec![i16::MAX; num_of_vertices]; num_of_vertices];
    let mut index_map: HashMap<usize, String> = HashMap::new();

    for (key, value) in valves.iter() {
        let pos = valves_map.get(key).unwrap();
        matrix[*pos][*pos] = 0;

        for valve in value.valves.iter() {
            let neighbor_pos = valves_map.get(valve).unwrap();
            matrix[*pos][*neighbor_pos] = 1;
        }

        index_map.insert(*pos, key.to_string());
    }

    for k in 0..num_of_vertices {
        for i in 0..num_of_vertices {
            for j in 0..num_of_vertices {
                if matrix[i][k] < i16::MAX
                    && matrix[k][j] < i16::MAX
                    && matrix[i][k] + matrix[k][j] < matrix[i][j]
                {
                    matrix[i][j] = matrix[i][k] + matrix[k][j];
                }
            }
        }
    }

    (matrix, index_map)
}

fn greedy_algorithm(
    matrix: Vec<Vec<i16>>,
    index_map: HashMap<usize, String>,
    valves: HashMap<String, Valve>,
) -> i32 {
    // println!("{:?}\n", index_map);

    let valves_length = index_map.len();
    let mut opened_valves: HashSet<&str> = HashSet::new();
    let mut valve;

    let mut total = 0;
    let mut row = 0;
    let mut minutes = 30;

    while minutes != 0 {
        let mut max = 0;
        let mut max_index = 0;
        let mut max_distance = 1;
        let mut distance;

        // println!("===============================================");

        for index in 0..valves_length {
            valve = valves.get(index_map.get(&index).unwrap()).unwrap();

            distance = matrix[row][index] as i32;

            let score = match opened_valves.contains(&valve.id as &str) {
                true => 0,
                false => valve.flow_rate * (minutes - (distance + 1)),
            };

            // println!(
            //     "{} Option {} with a distance of {} and score of {}",
            //     index,
            //     valve.id,
            //     distance + 1,
            //     score
            // );

            if score > max && minutes - distance >= 0 {
                max = score;
                max_index = index;
                max_distance = distance + 1;
            }
        }

        total += max;
        row = max_index;
        minutes -= max_distance;

        valve = valves.get(index_map.get(&max_index).unwrap()).unwrap();
        opened_valves.insert(&valve.id);

        // println!("Minute {}", minutes);
        // println!("Opened Valves: {:?}", opened_valves);
        // println!("Total Pressure: {}", total);
        // println!("You are at valve {}", valve.id);
        // println!();
    }

    total
}

fn main() {
    let valves = parse_input(
        "Valve AW has flow rate=0; tunnels lead to valves DS, AA
Valve NT has flow rate=4; tunnels lead to valves AO, IT, AM, VZ
Valve FI has flow rate=0; tunnels lead to valves NK, RH
Valve NK has flow rate=13; tunnels lead to valves VZ, QE, FI
Valve ZB has flow rate=0; tunnels lead to valves IC, TX
Valve DS has flow rate=3; tunnels lead to valves ME, JY, OV, RA, AW
Valve JT has flow rate=0; tunnels lead to valves RA, OE
Valve OH has flow rate=0; tunnels lead to valves KT, AK
Valve OE has flow rate=9; tunnels lead to valves SH, MR, JT, QI
Valve CT has flow rate=0; tunnels lead to valves JH, NA
Valve CB has flow rate=0; tunnels lead to valves XC, JH
Valve EK has flow rate=0; tunnels lead to valves GB, ZZ
Valve NA has flow rate=0; tunnels lead to valves GL, CT
Valve JY has flow rate=0; tunnels lead to valves DS, IH
Valve RA has flow rate=0; tunnels lead to valves JT, DS
Valve QT has flow rate=0; tunnels lead to valves ZG, KM
Valve SM has flow rate=0; tunnels lead to valves AK, AM
Valve XC has flow rate=11; tunnel leads to valve CB
Valve BF has flow rate=10; tunnels lead to valves BU, MR
Valve OV has flow rate=0; tunnels lead to valves BV, DS
Valve GB has flow rate=25; tunnel leads to valve EK
Valve SD has flow rate=0; tunnels lead to valves JF, CN
Valve IH has flow rate=0; tunnels lead to valves JY, KM
Valve DF has flow rate=0; tunnels lead to valves ON, IC
Valve BV has flow rate=6; tunnels lead to valves OV, JN, ZG, UF
Valve PO has flow rate=0; tunnels lead to valves AK, QE
Valve JH has flow rate=12; tunnels lead to valves CB, MI, CT
Valve CN has flow rate=22; tunnel leads to valve SD
Valve JF has flow rate=0; tunnels lead to valves KM, SD
Valve QI has flow rate=0; tunnels lead to valves MI, OE
Valve JN has flow rate=0; tunnels lead to valves BV, BS
Valve TX has flow rate=0; tunnels lead to valves KM, ZB
Valve ME has flow rate=0; tunnels lead to valves VG, DS
Valve ON has flow rate=0; tunnels lead to valves DF, AA
Valve GL has flow rate=20; tunnel leads to valve NA
Valve AA has flow rate=0; tunnels lead to valves ON, UF, WR, ML, AW
Valve BS has flow rate=0; tunnels lead to valves JN, IC
Valve RH has flow rate=0; tunnels lead to valves FI, KT
Valve BU has flow rate=0; tunnels lead to valves BF, BG
Valve IT has flow rate=0; tunnels lead to valves NT, KT
Valve MR has flow rate=0; tunnels lead to valves OE, BF
Valve AO has flow rate=0; tunnels lead to valves ML, NT
Valve KM has flow rate=16; tunnels lead to valves WR, IH, QT, TX, JF
Valve ML has flow rate=0; tunnels lead to valves AO, AA
Valve VG has flow rate=0; tunnels lead to valves ME, IC
Valve MI has flow rate=0; tunnels lead to valves QI, JH
Valve AM has flow rate=0; tunnels lead to valves NT, SM
Valve KT has flow rate=23; tunnels lead to valves BG, OH, RH, SH, IT
Valve AK has flow rate=14; tunnels lead to valves SM, PO, OH
Valve BG has flow rate=0; tunnels lead to valves KT, BU
Valve QE has flow rate=0; tunnels lead to valves NK, PO
Valve IC has flow rate=17; tunnels lead to valves VG, ZZ, BS, ZB, DF
Valve UF has flow rate=0; tunnels lead to valves BV, AA
Valve SH has flow rate=0; tunnels lead to valves KT, OE
Valve WR has flow rate=0; tunnels lead to valves AA, KM
Valve ZZ has flow rate=0; tunnels lead to valves IC, EK
Valve ZG has flow rate=0; tunnels lead to valves BV, QT
Valve VZ has flow rate=0; tunnels lead to valves NK, NT",
    );

    let matrix = floyd_warshall_algorithm(&valves.0, &valves.1);
    // print_matrix(&matrix.0, &valves.1);

    let now = Instant::now();
    println!("Result: {}", greedy_algorithm(matrix.0, matrix.1, valves.0));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
