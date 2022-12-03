use std::collections::HashSet;

fn map_to_score(val: char) -> i32 {
    if val.is_lowercase() {
        return ((val as u32) - 96) as i32;
    }

    ((val as u32) - 38) as i32
}

fn find_common_char(line: String) -> Option<char> {
    let mut set: HashSet<char> = HashSet::new();

    let chars = line.trim().chars().collect::<Vec<char>>();
    let first_half = &chars[0..chars.len() / 2];
    let second_half = &chars[chars.len() / 2..];

    for i in first_half {
        if !set.contains(i) {
            set.insert(*i);
        }
    }

    for i in second_half {
        if set.contains(i) {
            return Some(*i);
        }
    }

    None
}

fn calculate_sum(input: String) -> i32 {
    let mut total = 0;
    for i in input.lines() {
        if let Some(x) = find_common_char(i.trim().to_string()) {
            total += map_to_score(x);
        }
    }
    total
}
