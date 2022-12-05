fn arrange_stacks(input: &str) -> String {
    let mut result: String = "".to_string();

    let mut stacks: Vec<Vec<char>> = vec![
        ['Z', 'N'].to_vec(),
        ['M', 'C', 'D'].to_vec(),
        ['P'].to_vec(),
    ];

    let moves = input
        .lines()
        .map(|x| x.trim().split(' ').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    for action in moves {
        let amount: i32 = action[1].parse::<i32>().unwrap();
        let from: usize = (action[3].parse::<i32>().unwrap() - 1).try_into().unwrap();
        let to: usize = (action[5].parse::<i32>().unwrap() - 1).try_into().unwrap();

        for _i in 0..amount {
            if let Some(x) = stacks[from].pop() {
                stacks[to].push(x);
            }
        }
    }

    for stack in stacks {
        if let Some(x) = stack.last() {
            result.push(*x);
        }
    }

    result
}

//================================================================================================

fn arrange_stacks(input: &str) -> String {
    let mut result: String = "".to_string();

    let mut stacks: Vec<Vec<char>> = vec![
        // ['Z', 'N'].to_vec(),
        // ['M', 'C', 'D'].to_vec(),
        // ['P'].to_vec(),
        ['G', 'T', 'R', 'W'].to_vec(),
        ['G', 'C', 'H', 'P', 'M', 'S', 'V', 'W'].to_vec(),
        ['C', 'L', 'T', 'S', 'G', 'M'].to_vec(),
        ['J', 'H', 'D', 'M', 'W', 'R', 'F'].to_vec(),
        ['P', 'Q', 'L', 'H', 'S', 'W', 'F', 'J'].to_vec(),
        ['P', 'J', 'D', 'N', 'F', 'M', 'S'].to_vec(),
        ['Z', 'B', 'D', 'F', 'G', 'C', 'S', 'J'].to_vec(),
        ['R', 'T', 'B'].to_vec(),
        ['H', 'N', 'W', 'L', 'C'].to_vec(),
    ];

    let moves = input
        .lines()
        .map(|x| x.trim().split(' ').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    for action in moves {
        let amount: usize = action[1].parse::<i32>().unwrap().try_into().unwrap();
        let from: usize = (action[3].parse::<i32>().unwrap() - 1).try_into().unwrap();
        let to: usize = (action[5].parse::<i32>().unwrap() - 1).try_into().unwrap();

        let stack_size = stacks[from].len();
        let mut cargo = stacks[from]
            .drain((stack_size - amount)..)
            .collect::<Vec<char>>();
        stacks[to].append(&mut cargo);
    }

    for stack in stacks {
        if let Some(x) = stack.last() {
            result.push(*x);
        }
    }

    result
}
