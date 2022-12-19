use core::time;
use std::thread;

use colored::*;

const LIMIT: i16 = 494;
const PADDING: i16 = 2;
const START: [usize; 2] = [0, 8];

fn parse_input(input: &str) -> (Vec<Vec<[usize; 2]>>, usize, usize) {
    let mut coordinates: Vec<Vec<[usize; 2]>> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for line in input.lines() {
        let mut structure: Vec<[usize; 2]> = Vec::new();
        let parts: Vec<&str> = line.split("->").collect();

        let mut start: Vec<i16> = parts[0]
            .trim()
            .split(',')
            .map(|x| x.parse::<i16>().unwrap())
            .collect();

        structure.push([(start[0] - LIMIT + PADDING) as usize, start[1] as usize]);

        for item in parts.iter().skip(1) {
            let goal: Vec<i16> = item
                .trim()
                .split(',')
                .map(|x| x.parse::<i16>().unwrap())
                .collect();

            let mut difference = [start[0] - goal[0], start[1] - goal[1]];

            while difference != [0, 0] {
                let x_direction = match difference[0] {
                    value if value < 0 => 1,
                    value if value > 0 => -1,
                    _ => 0,
                };

                let y_direction = match difference[1] {
                    value if value < 0 => 1,
                    value if value > 0 => -1,
                    _ => 0,
                };

                start[0] += x_direction;
                difference[0] += x_direction;

                start[1] += y_direction;
                difference[1] += y_direction;

                structure.push([(start[0] - LIMIT + PADDING) as usize, start[1] as usize]);
            }

            if max_x < start[0] {
                max_x = start[0];
            }
            if max_y < start[1] {
                max_y = start[1];
            }
        }

        coordinates.push(structure);
    }

    (
        coordinates,
        (max_x + 4 - LIMIT + PADDING) as usize,
        (max_y + 3) as usize,
    )
}

fn print_cave(cave: &[Vec<char>]) {
    let mut picture: Vec<Vec<ColoredString>> = Vec::new();

    for row in cave.iter() {
        let mut colored_values: Vec<ColoredString> = Vec::new();

        for character in row {
            match character {
                'o' | '+' => colored_values.push(character.to_string().truecolor(252, 163, 17)),
                '#' => colored_values.push(character.to_string().truecolor(229, 229, 229)),
                _ => colored_values.push(character.to_string().white()),
            }
        }

        picture.push(colored_values);
    }

    for row in picture {
        for value in row {
            print!("{}", value);
        }
        println!();
    }
}

fn plot_coordinates(
    coordinates: Vec<Vec<[usize; 2]>>,
    right_limit: usize,
    bot_limit: usize,
) -> Vec<Vec<char>> {
    let mut cave: Vec<Vec<char>> = Vec::new();

    for _i in 0..bot_limit {
        let air: Vec<char> = vec!['.'; right_limit];
        cave.push(air);
    }

    for structure in coordinates {
        for part in structure {
            cave[part[1]][part[0]] = '#';
        }
    }

    cave[START[0]][START[1]] = '+';

    cave
}

fn is_in_ditch(cave: &[Vec<char>], pos: &[usize; 2], direction: i16) -> bool {
    let y_pos = pos[1] as i16 + direction;
    cave[pos[0]][y_pos as usize] == '#'
        && cave[pos[0] + 1][y_pos as usize] == '#'
        && cave[pos[0] + 1][pos[1]] == '#'
}

fn reservoir(mut cave: Vec<Vec<char>>) -> i16 {
    let mut sand_position: [usize; 2] = [START[0], START[1]];
    let mut at_rest = false;
    let mut num_at_rest: i16 = 0;

    loop {
        if sand_position[0] + 1 >= cave.len() || sand_position[1] + 1 >= cave[0].len() {
            break;
        }

        // middle is empty
        if cave[sand_position[0] + 1][sand_position[1]] == '.' {
            sand_position[0] += 1;
        }
        // left is empty
        else if cave[sand_position[0] + 1][sand_position[1] - 1] == '.' {
            sand_position[0] += 1;
            sand_position[1] -= 1;

            if is_in_ditch(&cave, &sand_position, -1) {
                at_rest = true;
            }
        }
        // right is empty
        else if cave[sand_position[0] + 1][sand_position[1] + 1] == '.' {
            sand_position[0] += 1;
            sand_position[1] += 1;

            if is_in_ditch(&cave, &sand_position, 1) {
                at_rest = true;
            }
        }
        // no positions are empty
        else {
            at_rest = true;
        }

        // sand_position[0] + 1 goes down
        // sand_position[1] + 1 goes right

        cave[sand_position[0]][sand_position[1]] = 'o';

        // print_cave(&cave);

        if at_rest {
            num_at_rest += 1;
            at_rest = false;
            sand_position = [START[0], START[1]];
        } else {
            cave[sand_position[0]][sand_position[1]] = '.';
        }

        // thread::sleep(time::Duration::from_micros(500));
        // clearscreen::clear().unwrap();
    }

    num_at_rest
}

fn main() {
    let parsed_input: (Vec<Vec<[usize; 2]>>, usize, usize) = parse_input(
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
    );

    let cave = plot_coordinates(parsed_input.0, parsed_input.1, parsed_input.2);

    println!("Result: {}", reservoir(cave));
}

//================================================================================================

use core::time;
use std::thread;

use colored::*;

const LIMIT: i16 = 494;
const START: [usize; 2] = [0, 8];
const PADDING: i16 = 2;

fn parse_input(input: &str) -> (Vec<Vec<[usize; 2]>>, usize, usize) {
    let mut coordinates: Vec<Vec<[usize; 2]>> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for line in input.lines() {
        let mut structure: Vec<[usize; 2]> = Vec::new();
        let parts: Vec<&str> = line.split("->").collect();

        let mut start: Vec<i16> = parts[0]
            .trim()
            .split(',')
            .map(|x| x.parse::<i16>().unwrap())
            .collect();

        structure.push([(start[0] - LIMIT + PADDING) as usize, start[1] as usize]);

        for item in parts.iter().skip(1) {
            let goal: Vec<i16> = item
                .trim()
                .split(',')
                .map(|x| x.parse::<i16>().unwrap())
                .collect();

            let mut difference = [start[0] - goal[0], start[1] - goal[1]];

            while difference != [0, 0] {
                let x_direction = match difference[0] {
                    value if value < 0 => 1,
                    value if value > 0 => -1,
                    _ => 0,
                };

                let y_direction = match difference[1] {
                    value if value < 0 => 1,
                    value if value > 0 => -1,
                    _ => 0,
                };

                start[0] += x_direction;
                difference[0] += x_direction;

                start[1] += y_direction;
                difference[1] += y_direction;

                structure.push([(start[0] - LIMIT + PADDING) as usize, start[1] as usize]);
            }

            if max_x < start[0] {
                max_x = start[0];
            }
            if max_y < start[1] {
                max_y = start[1];
            }
        }

        coordinates.push(structure);
    }

    (
        coordinates,
        (max_x + 4 - LIMIT + PADDING) as usize,
        (max_y + 3) as usize,
    )
}

fn print_cave(cave: &[Vec<char>]) {
    let mut picture: Vec<Vec<ColoredString>> = Vec::new();

    for row in cave.iter() {
        let mut colored_values: Vec<ColoredString> = Vec::new();

        for character in row {
            match character {
                'o' | '+' => colored_values.push(character.to_string().truecolor(252, 163, 17)),
                '#' => colored_values.push(character.to_string().truecolor(229, 229, 229)),
                _ => colored_values.push(character.to_string().white()),
            }
        }

        picture.push(colored_values);
    }

    for row in picture {
        for value in row {
            print!("{}", value);
        }
        println!();
    }
}

fn plot_coordinates(
    coordinates: Vec<Vec<[usize; 2]>>,
    right_limit: usize,
    bot_limit: usize,
) -> Vec<Vec<char>> {
    let mut cave: Vec<Vec<char>> = Vec::new();
    let difference = right_limit / 2;
    let padding_left = difference - START[1];

    for _i in 0..bot_limit - 1 {
        let air: Vec<char> = vec!['.'; right_limit];
        cave.push(air);
    }

    let floor: Vec<char> = vec!['#'; right_limit];
    cave.push(floor);

    for structure in coordinates {
        for part in structure {
            cave[part[1]][part[0]+padding_left] = '#';
        }
    }

    cave[START[0]][START[1] + padding_left] = '+';


    cave
}

fn is_in_ditch(cave: &[Vec<char>], pos: &[usize; 2], direction: i16) -> bool {
    let y_pos = pos[1] as i16 + direction;
    cave[pos[0]][y_pos as usize] == '#'
        && cave[pos[0] + 1][y_pos as usize] == '#'
        && cave[pos[0] + 1][pos[1]] == '#'
}

fn reservoir(cave: &mut Vec<Vec<char>>) -> i16 {
    let difference = cave[0].len() / 2;
    let padding_left = difference - START[1];

    let mut sand_position: [usize; 2] = [START[0], START[1] + padding_left];
    let mut at_rest = false;
    let mut num_at_rest: i16 = 0;

    loop {
        if sand_position[0] + 1 >= cave.len() 
        || sand_position[1] + 1 >= cave[0].len()
        {
            break;
        }

        // middle is empty
        if cave[sand_position[0] + 1][sand_position[1]] == '.' {
            sand_position[0] += 1;
        }
        // left is empty
        else if cave[sand_position[0] + 1][sand_position[1] - 1] == '.' {
            sand_position[0] += 1;
            sand_position[1] -= 1;

            if sand_position[0] + 1 >= cave.len() && is_in_ditch(&cave, &sand_position, -1) {
                at_rest = true;
            }
        }
        // right is empty
        else if cave[sand_position[0] + 1][sand_position[1] + 1] == '.' {
            sand_position[0] += 1;
            sand_position[1] += 1;

            if sand_position[0] + 1 >= cave.len() && is_in_ditch(&cave, &sand_position, 1) {
                at_rest = true;
            }
        }
        // no positions are empty
        else {
            at_rest = true;
        }

        // sand_position[0] + 1 goes down
        // sand_position[1] + 1 goes right

        cave[sand_position[0]][sand_position[1]] = 'o';

        // print_cave(&cave);

        if at_rest {
            num_at_rest += 1;

            if sand_position == [START[0], START[1] + padding_left] {
                break;
            }

            at_rest = false;
            sand_position = [START[0], START[1] + padding_left];
        } else {
            cave[sand_position[0]][sand_position[1]] = '.';
        }

        // thread::sleep(time::Duration::from_millis(50));
        // print! ("\x1B[2J\x1B[1;1H");

        // clearscreen::clear().unwrap();
    }

    num_at_rest
}

fn main() {
    let parsed_input: (Vec<Vec<[usize; 2]>>, usize, usize) = parse_input(
        "",
    );

    let mut cave = plot_coordinates(parsed_input.0, parsed_input.1 * 5, parsed_input.2);


    println!("Result: {}", reservoir(&mut cave));
    print_cave(&cave);
}

