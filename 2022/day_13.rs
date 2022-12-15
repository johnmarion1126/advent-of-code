use std::cmp::Ordering;

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

// false == Ordering::Greater
// true == Ordering::Less
// equal == Ordering::Equal

fn in_correct_order(packet_1: &Vec<Packet>, packet_2: &Vec<Packet>) -> Ordering {
    let mut index = 0;

    while index < packet_1.len() || index < packet_2.len() {
        match (&packet_1.get(index), &packet_2.get(index)) {
            (_, None) => {
                return Ordering::Greater;
            }

            (None, _) => {
                return Ordering::Less;
            }

            (Some(Packet::Num(val_1)), Some(Packet::Num(val_2))) => {
                if val_1 < val_2 {
                    return Ordering::Less;
                } else if val_1 > val_2 {
                    return Ordering::Greater;
                }
            }

            (Some(Packet::List(list_1)), Some(Packet::Num(val_2))) => {
                let result = in_correct_order(list_1, &vec![Packet::Num(*val_2)]);
                if result != Ordering::Equal {
                    return result;
                }
            }

            (Some(Packet::Num(val_1)), Some(Packet::List(list_2))) => {
                let result = in_correct_order(&vec![Packet::Num(*val_1)], list_2);
                if result != Ordering::Equal {
                    return result;
                }
            }

            (Some(Packet::List(list_1)), Some(Packet::List(list_2))) => {
                let result = in_correct_order(list_1, list_2); 
                if result != Ordering::Equal {
                    return result;
                }
            }
        }

        index += 1;
    }

    Ordering::Equal
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

        // If left is less than right
        // Increment result

        // If left is greater than right or if left and right is equal
        // Don't do anything

        if let Ordering::Less = in_correct_order(&packets[index], &packets[index + 1]) {
            result += counter;
        }
    }

    println!("Result: {}", result);
}


//================================================================================================

use std::cmp::Ordering;

#[derive(Debug)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

#[derive(Debug)]
struct PacketStruct {
    packets: Vec<Packet>
}

impl PacketStruct {
    fn new(packets: Vec<Packet>) -> Self {
        Self {
            packets
        }
    }

    fn in_correct_order(packet_1: &Vec<Packet>, packet_2: &Vec<Packet>) -> Ordering {
        let mut index = 0;

        while index < packet_1.len() || index < packet_2.len() {
            match (&packet_1.get(index), &packet_2.get(index)) {
                (_, None) => {
                    return Ordering::Greater;
                }

                (None, _) => {
                    return Ordering::Less;
                }

                (Some(Packet::Num(val_1)), Some(Packet::Num(val_2))) => {
                    if val_1 < val_2 {
                        return Ordering::Less;
                    } else if val_1 > val_2 {
                        return Ordering::Greater;
                    }
                }

                (Some(Packet::List(list_1)), Some(Packet::Num(val_2))) => {
                    let result = in_correct_order(list_1, &vec![Packet::Num(*val_2)]);
                    if result != Ordering::Equal {
                        return result;
                    }
                }

                (Some(Packet::Num(val_1)), Some(Packet::List(list_2))) => {
                    let result = in_correct_order(&vec![Packet::Num(*val_1)], list_2);
                    if result != Ordering::Equal {
                        return result;
                    }
                }

                (Some(Packet::List(list_1)), Some(Packet::List(list_2))) => {
                    let result = in_correct_order(list_1, list_2); 
                    if result != Ordering::Equal {
                        return result;
                    }
                }
            }

            index += 1;
        }

        Ordering::Equal
    }

    fn compare(&self, other: &Vec<Packet>) -> Ordering {
        in_correct_order(&self.packets, other)
    }

}

impl Ord for PacketStruct {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(&other.packets)
    }
}

impl PartialOrd for PacketStruct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(&other.packets))
    }
}

impl PartialEq for PacketStruct {
    fn eq(&self, other: &Self) -> bool {
        if self.compare(&other.packets) == Ordering::Equal {
            return true;
        }
        false
    }
}

impl Eq for PacketStruct {}

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

// false == Ordering::Greater
// true == Ordering::Less
// equal == Ordering::Equal

fn in_correct_order(packet_1: &Vec<Packet>, packet_2: &Vec<Packet>) -> Ordering {
    let mut index = 0;

    while index < packet_1.len() || index < packet_2.len() {
        match (&packet_1.get(index), &packet_2.get(index)) {
            (_, None) => {
                return Ordering::Greater;
            }

            (None, _) => {
                return Ordering::Less;
            }

            (Some(Packet::Num(val_1)), Some(Packet::Num(val_2))) => {
                if val_1 < val_2 {
                    return Ordering::Less;
                } else if val_1 > val_2 {
                    return Ordering::Greater;
                }
            }

            (Some(Packet::List(list_1)), Some(Packet::Num(val_2))) => {
                let result = in_correct_order(list_1, &vec![Packet::Num(*val_2)]);
                if result != Ordering::Equal {
                    return result;
                }
            }

            (Some(Packet::Num(val_1)), Some(Packet::List(list_2))) => {
                let result = in_correct_order(&vec![Packet::Num(*val_1)], list_2);
                if result != Ordering::Equal {
                    return result;
                }
            }

            (Some(Packet::List(list_1)), Some(Packet::List(list_2))) => {
                let result = in_correct_order(list_1, list_2); 
                if result != Ordering::Equal {
                    return result;
                }
            }
        }

        index += 1;
    }

    Ordering::Equal
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
[1,[2,[3,[4,[5,6,0]]]],8,9]
",
    );

    let mut packet_list: Vec<PacketStruct> = Vec::new();

    for i in packets {
        packet_list.push(PacketStruct::new(i));
    }

    let divider_1 = PacketStruct::new(vec!(Packet::List(vec!(Packet::Num(2)))));
    let divider_2 = PacketStruct::new(vec!(Packet::List(vec!(Packet::Num(6)))));

    packet_list.push(divider_1);
    packet_list.push(divider_2);

    packet_list.sort();

    let target_1 = PacketStruct::new(vec!(Packet::List(vec!(Packet::Num(2)))));
    let target_2 = PacketStruct::new(vec!(Packet::List(vec!(Packet::Num(6)))));

    let mut index_1 = 1;
    let mut index_2 = 1;


    let mut index = 1;
    for i in packet_list {
        if i == target_1 {
            index_1 = index;
        }
        if i == target_2 {
            index_2 = index;
        }
        index += 1;
    }

    println!("Result: {}", index_1 * index_2);
}

