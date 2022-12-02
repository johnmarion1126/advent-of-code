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

//================================================================================================

fn get_lose_option(option: &str) -> i32 {
    match option {
        "A" => 3, // Z
        "B" => 1, // X
        "C" => 2, // Y
        _ => 0,
    }
}

fn get_win_option(option: &str) -> i32 {
    match option {
        "A" => 2, // Y
        "B" => 3, // Z
        "C" => 1, // X
        _ => 0,
    }
}

fn get_draw_option(option: &str) -> i32 {
    match option {
        "A" => 1, // Y
        "B" => 2, // Z
        "C" => 3, // X
        _ => 0,
    }
}

fn calculate_score(opps: &str, player: &str) -> i32 {
    match player {
        "X" => get_lose_option(opps),
        "Y" => 3 + get_draw_option(opps),
        "Z" => 6 + get_win_option(opps),
        _ => 0,
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
