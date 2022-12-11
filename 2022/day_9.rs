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

//================================================================================================

use std::collections::HashSet;

struct Rope {
    position: [i32; 2],
}

impl Rope {
    fn new(x: i32, y: i32) -> Self {
        Self { position: [x, y] }
    }

    fn update_position(&mut self, x: i32, y: i32) {
        match [x, y] {
            [x, y] if self.position[0] + -1 == x && self.position[1] + 2 == y => {
                self.position[0] += -1;
                self.position[1] += 1;
            }
            [x, y] if self.position[0] == x && self.position[1] + 2 == y => {
                self.position[1] += 1;
            }
            [x, y] if self.position[0] + 1 == x && self.position[1] + 2 == y => {
                self.position[0] += 1;
                self.position[1] += 1;
            }
            [x, y] if self.position[0] + 2 == x && self.position[1] == y => {
                self.position[0] += 1;
            }
            [x, y] if self.position[0] + 1 == x && self.position[1] - 2 == y => {
                self.position[0] += 1;
                self.position[1] += -1;
            }
            [x, y] if self.position[0] == x && self.position[1] - 2 == y => {
                self.position[1] += -1;
            }
            [x, y] if self.position[0] + -1 == x && self.position[1] - 2 == y => {
                self.position[0] += -1;
                self.position[1] += -1;
            }
            [x, y] if self.position[0] + -2 == x && self.position[1] == y => {
                self.position[0] += -1;
            }
            [x, y] if self.position[0] + -2 == x && self.position[1] + 1 == y => {
                self.position[0] += -1;
                self.position[1] += 1;
            }
            [x, y] if self.position[0] + -2 == x && self.position[1] - 1 == y => {
                self.position[0] += -1;
                self.position[1] += -1;
            }
            [x, y] if self.position[0] + 2 == x && self.position[1] + 1 == y => {
                self.position[0] += 1;
                self.position[1] += 1;
            }
            [x, y] if self.position[0] + 2 == x && self.position[1] - 1 == y => {
                self.position[0] += 1;
                self.position[1] += -1;
            }

            [x, y] if self.position[0] + 2 == x && self.position[1] + 2 == y => {
                self.position[0] += 1;
                self.position[1] += 1;
            }
            [x, y] if self.position[0] + 2 == x && self.position[1] - 2 == y => {
                self.position[0] += 1;
                self.position[1] += -1;
            }
            [x, y] if self.position[0] - 2 == x && self.position[1] + 2 == y => {
                self.position[0] -= 1;
                self.position[1] += 1;
            }
            [x, y] if self.position[0] - 2 == x && self.position[1] - 2 == y => {
                self.position[0] += -1;
                self.position[1] += -1;
            }
            _ => {}
        }
    }
}

fn total_visited_pos(input: &str) -> usize {
    let mut rope_1 = Rope::new(0, 0);
    let mut rope_2 = Rope::new(0, 0);
    let mut rope_3 = Rope::new(0, 0);
    let mut rope_4 = Rope::new(0, 0);
    let mut rope_5 = Rope::new(0, 0);
    let mut rope_6 = Rope::new(0, 0);
    let mut rope_7 = Rope::new(0, 0);
    let mut rope_8 = Rope::new(0, 0);
    let mut rope_9 = Rope::new(0, 0);

    let mut head = [0, 0];
    let mut positions: HashSet<String> = HashSet::new();

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

        for _i in 0..distance {
            head[axis] += direction;

            rope_1.update_position(head[0], head[1]);

            rope_2.update_position(rope_1.position[0], rope_1.position[1]);

            rope_3.update_position(rope_2.position[0], rope_2.position[1]);

            rope_4.update_position(rope_3.position[0], rope_3.position[1]);

            rope_5.update_position(rope_4.position[0], rope_4.position[1]);

            rope_6.update_position(rope_5.position[0], rope_5.position[1]);

            rope_7.update_position(rope_6.position[0], rope_6.position[1]);

            rope_8.update_position(rope_7.position[0], rope_7.position[1]);

            rope_9.update_position(rope_8.position[0], rope_8.position[1]);

            positions.insert(format!("{}-{}", rope_9.position[0], rope_9.position[1]));
        }
    }

    positions.len()
}
