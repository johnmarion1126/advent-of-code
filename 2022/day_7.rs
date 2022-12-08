const THRESHOLD: i32 = 100000;

fn find_deletable_amount(input: &str) -> i32 {
    let mut path: Vec<i32> = Vec::new();
    let mut result = 0;

    for i in input.lines() {
        let command = i.split(" ").collect::<Vec<&str>>();

        if i.trim().contains("$ cd") {
            if let Some(folder_name) = command.get(2) {
                match folder_name {
                    &".." => {
                        if let Some(size) = path.pop() {
                            if size < THRESHOLD {
                                result += size;
                            }
                            *path.last_mut().unwrap() += size;
                        }
                    }
                    _ => {
                        path.push(0);
                    }
                }
            }
        } else if let Ok(size) = command[0].parse::<i32>() {
            *path.last_mut().unwrap() += size;
        }
    }

    while path.len() != 1 {
        if let Some(size) = path.pop() {
            if size < THRESHOLD {
                result += size;
            }
            *path.last_mut().unwrap() += size;
        }
    }

    result
}
