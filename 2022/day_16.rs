use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

#[derive(Debug)]
struct Valve {
    id: String,
    index: usize,
    flow_rate: i32,
    valves: Vec<String>,
}

impl Valve {
    fn new(id: String, index: usize, flow_rate: i32, valves: Vec<String>) -> Self {
        Self {
            id,
            index,
            flow_rate,
            valves,
        }
    }
}

fn parse_input(input: &str) -> (HashMap<usize, Valve>, HashMap<String, usize>) {
    let mut valves: HashMap<usize, Valve> = HashMap::new();
    let mut index_map: HashMap<String, usize> = HashMap::new();

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

        let valves_len = valves.len();
        let valve = Valve::new(
            id.to_string(),
            valves_len,
            flow_rate.parse::<i32>().unwrap(),
            ids,
        );

        valves.insert(valves_len, valve);
        index_map.insert(id.to_string(), valves_len);
    }

    (valves, index_map)
}

fn print_matrix(matrix: &[Vec<i16>], index_map: &HashMap<String, usize>) {
    println!();
    print!("    ");
    let mut labels: Vec<String> = vec![String::from("00"); index_map.len()];
    for (key, value) in index_map.iter() {
        labels[*value] = key.to_string();
    }

    for label in labels.iter() {
        print!("{0: <7}", label);
    }

    println!();

    for index in 0..index_map.len() {
        print!("{}  ", labels[index]);
        for item in &matrix[index] {
            print!("{0: <7}", item);
        }

        println!();
    }

    println!();
}

fn floyd_warshall_algorithm(
    valves: &HashMap<usize, Valve>,
    index_map: &HashMap<String, usize>,
) -> Vec<Vec<i16>> {
    let num_of_vertices = valves.len();
    let mut matrix: Vec<Vec<i16>> = vec![vec![i16::MAX; num_of_vertices]; num_of_vertices];

    for (_key, value) in valves.iter() {
        let pos = value.index;
        matrix[pos][pos] = 0;

        for valve in value.valves.iter() {
            let neighbor_pos = index_map.get(valve).unwrap();
            matrix[pos][*neighbor_pos] = 1;
        }
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

    matrix
}

fn dfs(
    minutes: i32,
    total: i32,
    row: usize,
    valve: &Valve,
    matrix: &Vec<Vec<i16>>,
    valves_map: &HashMap<usize, Valve>,
    mut opened_values: HashSet<String>,
) -> i32 {
    // println!("MINUTES {} SCORE {}", minutes, total);
    if minutes == 0 {
        return total;
    }

    opened_values.insert(valve.id.to_string());

    let mut max = 0;

    for column in 0..matrix[row].len() {
        let valve = valves_map.get(&column).unwrap();

        let distance = matrix[row][column] as i32 + 1;

        if minutes - distance < 0 || opened_values.contains(&valve.id as &str) {
            continue;
        }

        let total_flow_rate = valve.flow_rate * (minutes - distance);

        let score = dfs(
            minutes - distance,
            total + total_flow_rate,
            column,
            valve,
            matrix,
            valves_map,
            opened_values.clone(),
        );

        if max < score {
            max = score;
        }
    }

    max
}

fn dfs_helper(matrix: Vec<Vec<i16>>, valves_map: HashMap<usize, Valve>) -> i32 {
    let opened_values: HashSet<String> = HashSet::new();
    dfs(
        30,
        0,
        0,
        &valves_map.get(&0).unwrap(),
        &matrix,
        &valves_map,
        opened_values,
    )
}

fn main() {
    let valves = parse_input(
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
    );

    let matrix = floyd_warshall_algorithm(&valves.0, &valves.1);
    // print_matrix(&matrix, &valves.1);

    // let now = Instant::now();
    println!("Result: {}", dfs_helper(matrix, valves.0));
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
