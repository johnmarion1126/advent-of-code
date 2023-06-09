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

//================================================================================================

const TOTAL_DISK_SPACE: i32 = 70000000;
const REQUIRED_SPACE: i32 = 30000000;

fn binary_search(val: i32, items: &Vec<i32>) -> usize {
    if items.is_empty() {
        return 0;
    }

    let mut low = 0;
    let mut high = items.len() - 1;

    while low < high {
        let mid = low + (high - low) / 2;

        if let Some(curr) = items.get(mid as usize) {
            if *curr <= val {
                low = (mid + 1) as usize;
            } else {
                high = mid;
            }
        }
    }

    if items[low] < val {
        low += 1;
    }

    low
}

fn find_deletable_amount(input: &str) -> i32 {
    let mut path: Vec<i32> = Vec::new();
    let mut all_values: Vec<i32> = Vec::new();

    for i in input.lines() {
        let command = i.split(" ").collect::<Vec<&str>>();

        if i.trim().contains("$ cd") {
            if let Some(folder_name) = command.get(2) {
                match folder_name {
                    &".." => {
                        if let Some(size) = path.pop() {
                            *path.last_mut().unwrap() += size;
                            all_values.insert(binary_search(size, &all_values), size);
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
            *path.last_mut().unwrap() += size;
            all_values.insert(binary_search(size, &all_values), size);
        }
    }

    let needed_space = REQUIRED_SPACE - (TOTAL_DISK_SPACE - path.pop().unwrap());
    all_values[binary_search(needed_space, &all_values)]
}
