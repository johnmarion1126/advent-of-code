fn check_cycle(signal: i32, cycles: i32) -> i32 {
    if cycles == 20
        || cycles == 60
        || cycles == 100
        || cycles == 140
        || cycles == 180
        || cycles == 220
    {
        // println!("CYCLE {} SIGNAL {}", cycles, signal);
        return signal * cycles;
    }
    0
}

fn get_total(input: &str) -> i32 {
    let mut cycles = 0;
    let mut total_signal = 0;
    let mut signal = 1;

    for i in input.lines() {
        let parts: Vec<_> = i.trim().split(" ").collect();
        let mut value = 0;

        cycles += 1;
        total_signal += check_cycle(signal, cycles);

        match parts[..] {
            ["addx", val] => {
                // println!("ADDX");
                value = val.parse::<i32>().unwrap();
            }
            ["noop"] => {
                // println!("NOOP");
                continue;
            }
            _ => {}
        }

        cycles += 1;
        total_signal += check_cycle(signal, cycles);

        signal += value;
    }

    total_signal
}

//================================================================================================

fn draw_message(input: &str) {
    let mut crt_idx = 0;
    let mut sprite_idx = 0;
    let mut row = "".to_string();

    for i in input.lines() {
        let parts: Vec<_> = i.trim().split(' ').collect();
        let mut value = 0;

        if crt_idx == 40 {
            println!("{}", row);
            row = "".to_string();
            crt_idx = 0;
        }

        if crt_idx == sprite_idx || crt_idx == sprite_idx + 1 || crt_idx == sprite_idx + 2 {
            row.push('#');
        } else {
            row.push('.');
        }

        crt_idx += 1;

        if crt_idx == 40 {
            println!("{}", row);
            row = "".to_string();
            crt_idx = 0;
        }

        match parts[..] {
            ["addx", val] => {
                // println!("ADDX");
                value = val.parse::<i32>().unwrap();
            }
            ["noop"] => {
                // println!("NOOP");
                continue;
            }
            _ => {}
        }


        if crt_idx == sprite_idx || crt_idx == sprite_idx + 1 || crt_idx == sprite_idx + 2 {
            row.push('#');
        } else {
            row.push('.');
        }

        crt_idx += 1;

        sprite_idx += value;

        // update position
    }
}
