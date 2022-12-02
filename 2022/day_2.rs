fn get_player_option_score(option: &str) -> i32 {
    match option {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn calculate_score(opps: &str, player: &str) -> i32 {
    match (opps, player) {
        ("A", "Z") | ("B", "X") | ("C", "Y") => get_player_option_score(player),
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3 + (get_player_option_score(player)),
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6 + (get_player_option_score(player)),
        (_, _) => 0,
    }
}

fn get_score(input: &str) -> i32 {
    let mut total = 0;

    for i in input.split("\n") {
        let combos = i.trim().split(" ").collect::<Vec<&str>>();
        if combos.len() != 2 {
            continue;
        }
        total += calculate_score(combos[0], combos[1]);
    }

    total
}
