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
