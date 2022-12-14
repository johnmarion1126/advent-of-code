use colored::*;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    thread, time::{self, Instant},
};

type Position = [usize; 2];

#[derive(Debug, Eq, PartialEq)]
struct Square {
    position: Position,
    score: i32,
    path: HashSet<Position>,
}

impl Square {
    fn new(position: Position, score: i32, path: HashSet<Position>) -> Self {
        Self {
            position,
            score,
            path,
        }
    }
}

impl Ord for Square {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Square {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Position, Position) {
    let mut map = Vec::new();
    let mut start: Position = [0, 0];
    let mut end: Position = [0, 0];

    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for character in line.trim().chars() {
            row.push(character);

            match character {
                'S' => {
                    start = [map.len(), row.len() - 1];
                }
                'E' => {
                    end = [map.len(), row.len() - 1];
                }
                _ => {}
            }
        }
        map.push(row);
    }

    (map, start, end)
}

// h(n) - new position to goal
fn manhattan_distance(new: Position, goal: Position) -> i32 {
    (new[0] as i32 - goal[0] as i32).abs() + (new[1] as i32 - goal[1] as i32).abs()
}

// g(n) - current position to new position
// elevation of the destination square can be at most one higher than the current square
fn is_viable_square(current_elevation: char, new_elevation: char) -> bool {
    let current_value = match current_elevation {
        'S' => 97,
        value => value as u16,
    };

    let new_value = match new_elevation {
        'E' => 122,
        value => value as u16,
    };

    current_value == new_value - 1 || current_value == new_value || current_value > new_value
}

// f(n) = g(n) + h(n)
fn a_star(map: Vec<Vec<char>>, start: Position, end: Position) -> usize {
    // Create priority queue of scores
    let mut squares = BinaryHeap::new();
    squares.push(Square {
        position: start,
        score: 1,
        path: HashSet::new(),
    });

    // Create a hashset of visited squares
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(start);

    // Create path
    let mut current_path: HashSet<Position> = HashSet::new();

    // Continue to iterate until we reach destination
    while let Some(Square {
        position,
        score,
        path,
    }) = squares.pop()
    {
        let x = position[0];
        let y = position[1];
        let elevation = map[x][y];
        let score = score;
        current_path = path;
        current_path.insert([x, y]);

        if [x, y] == end {
            break;
        }

        // Calculate up
        if x > 0
            && is_viable_square(elevation, map[x - 1][y])
            && !visited.contains(&[x - 1, y])
        {
            let square = Square::new(
                [x - 1, y],
                score + manhattan_distance([x - 1, y], end),
                current_path.clone(),
            );

            squares.push(square);
            visited.insert([x - 1, y]);
        }

        // Calculate down
        if x < map.len() - 1
            && is_viable_square(elevation, map[x + 1][y])
            && !visited.contains(&[x + 1, y])
        {
            let square = Square::new(
                [x + 1, y],
                score + manhattan_distance([x + 1, y], end),
                current_path.clone(),
            );

            squares.push(square);
            visited.insert([x + 1, y]);
        }

        // Calculate left
        if y > 0
            && is_viable_square(elevation, map[x][y - 1])
            && !visited.contains(&[x, y - 1])
        {
            let square = Square::new(
                [x, y - 1],
                score + manhattan_distance([x, y - 1], end),
                current_path.clone(),
            );

            squares.push(square);
            visited.insert([x, y - 1]);
        }

        // Calculate right
        if y < map[0].len() - 1
            && is_viable_square(elevation, map[x][y + 1])
            && !visited.contains(&[x, y + 1])
        {
            let square = Square::new(
                [x, y + 1],
                score + manhattan_distance([x, y + 1], end),
                current_path.clone(),
            );

            squares.push(square);
            visited.insert([x, y + 1]);
        }

        // Visuals
        
        // let mut test: Vec<Vec<ColoredString>> = Vec::new();
        // for a in map.iter() {
        //     let mut row: Vec<ColoredString> = Vec::new();
        //     for b in a {
        //         row.push(b.to_string().white());
        //     }
        //     test.push(row);
        // }

        // for i in current_path.iter() {
        //     test[i[0]][i[1]] = "#".red();
        // }

        // for row in test {
        //     for i in row {
        //         print!("{}", i);
        //     }
        //     println!();
        // }

        // if [x, y] == end {
        //     break;
        // }

        // thread::sleep(time::Duration::from_millis(100));
        // clearscreen::clear().unwrap();
    }

    // println!("{:?}", current_path);
    current_path.len() - 1
}

//================================================================================================
