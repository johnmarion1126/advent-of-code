use std::collections::HashSet;

fn total_visited_pos(input: &str) -> usize {
    let mut positions: HashSet<String> = HashSet::new();
    let mut head_pos = [0, 0];
    let mut tail_pos = [0, 0];

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let distance = parts[1].parse::<i16>().unwrap();

        let axis = match parts[0] {
            "U" | "D" => 1,
            _ => 0,
        };

        let direction = match parts[0] {
            "U" | "R" => 1,
            _ => -1,
        };

        // println!("\n{} {}", parts[0], parts[1]);

        for _i in 0..distance {
            head_pos[axis] += direction;
            // println!("{:?}", head_pos);

            match head_pos {
                [x, y] if tail_pos[0] + -1 == x && tail_pos[1] + 2 == y => {
                    tail_pos[0] += -1;
                    tail_pos[1] += 1;
                }
                [x, y] if tail_pos[0] == x && tail_pos[1] + 2 == y => {
                    tail_pos[1] += 1;
                }
                [x, y] if tail_pos[0] + 1 == x && tail_pos[1] + 2 == y => {
                    tail_pos[0] += 1;
                    tail_pos[1] += 1;
                }
                [x, y] if tail_pos[0] + 2 == x && tail_pos[1] == y => {
                    tail_pos[0] += 1;
                }
                [x, y] if tail_pos[0] + 1 == x && tail_pos[1] - 2 == y => {
                    tail_pos[0] += 1;
                    tail_pos[1] += -1;
                }
                [x, y] if tail_pos[0] == x && tail_pos[1] - 2 == y => {
                    tail_pos[1] += -1;
                }
                [x, y] if tail_pos[0] + -1 == x && tail_pos[1] - 2 == y => {
                    tail_pos[0] += -1;
                    tail_pos[1] += -1;
                }
                [x, y] if tail_pos[0] + -2 == x && tail_pos[1] == y => {
                    tail_pos[0] += -1;
                }
                [x, y] if tail_pos[0] + -2 == x && tail_pos[1] + 1 == y => {
                    tail_pos[0] += -1;
                    tail_pos[1] += 1;
                }
                [x, y] if tail_pos[0] + -2 == x && tail_pos[1] - 1 == y => {
                    tail_pos[0] += -1;
                    tail_pos[1] += -1;
                }

                [x, y] if tail_pos[0] + 2 == x && tail_pos[1] + 1 == y => {
                    tail_pos[0] += 1;
                    tail_pos[1] += 1;
                }
                [x, y] if tail_pos[0] + 2 == x && tail_pos[1] - 1 == y => {
                    tail_pos[0] += 1;
                    tail_pos[1] += -1;
                }
                _ => {}
            }

            // println!("TAIL {:?}", tail_pos);

            positions.insert(format!("{}-{}", tail_pos[0], tail_pos[1]));
        }
    }

    positions.len()
}
