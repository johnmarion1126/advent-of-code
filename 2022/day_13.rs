#[derive(Debug)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

fn parse_input(input: &str) -> Vec<Vec<Packet>> {
    let mut packets: Vec<Vec<Packet>> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let characters: Vec<char> = line.trim().chars().collect();
        packets.push(parse_list(characters[1..characters.len() - 1].to_vec()));
    }

    packets
}

fn parse_list(input: Vec<char>) -> Vec<Packet> {
    let mut packet: Vec<Packet> = Vec::new();
    let mut brackets: i16 = 0;
    let mut index = 0;
    let mut num = String::from("");

    while index < input.len() {
        match input[index] {
            '[' => {
                if let Ok(x) = num.parse::<u32>() {
                    packet.push(Packet::Num(x));
                    num = String::from("");
                }

                index += 1;
                brackets += 1;
                let start_index = index;

                while brackets > 0 {
                    match input[index] {
                        '[' => brackets += 1,
                        ']' => brackets -= 1,
                        _ => {}
                    }

                    index += 1;
                }

                packet.push(Packet::List(parse_list(
                    input[start_index..index - 1].to_vec(),
                )));
            }
            ',' | ']' => {
                if let Ok(x) = num.parse::<u32>() {
                    packet.push(Packet::Num(x));
                    num = String::from("");
                }
            }
            value => {
                num.push(value);
            }
        }

        index += 1;
    }

    if let Ok(x) = num.parse::<u32>() {
        packet.push(Packet::Num(x));
    }

    packet
}

fn in_correct_order(packet_1: &Vec<Packet>, packet_2: &Vec<Packet>) -> Option<bool> {
    let mut index = 0;

    while index < packet_1.len() || index < packet_2.len() {
        match (&packet_1.get(index), &packet_2.get(index)) {
            (_, None) => {
                return Some(false);
            }

            (None, _) => {
                return Some(true);
            }

            (Some(Packet::Num(val_1)), Some(Packet::Num(val_2))) => {
                if val_1 < val_2 {
                    return Some(true);
                } else if val_1 > val_2 {
                    return Some(false);
                }
            }

            (Some(Packet::List(list_1)), Some(Packet::Num(val_2))) => {
                if let Some(x) = in_correct_order(list_1, &vec![Packet::Num(*val_2)]) {
                    return Some(x);
                }
            }

            (Some(Packet::Num(val_1)), Some(Packet::List(list_2))) => {
                if let Some(x) = in_correct_order(&vec![Packet::Num(*val_1)], list_2) {
                    return Some(x);
                }
            }

            (Some(Packet::List(list_1)), Some(Packet::List(list_2))) => {
                if let Some(x) = in_correct_order(list_1, list_2) {
                    return Some(x);
                }
            }
        }

        index += 1;
    }

    None
}

fn main() {
    let packets = parse_input(
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
    );

    let mut result = 0;
    let mut counter = 0;

    for index in (0..packets.len() - 1).step_by(2) {
        counter += 1;

        if in_correct_order(&packets[index], &packets[index + 1]).unwrap() {
            result += counter;
        }
    }

    println!("Result: {}", result);
}

//================================================================================================
