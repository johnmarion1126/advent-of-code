fn find_elves_with_most_calories(input: String) -> i32 {
    let mut max = 0;
    let mut total = 0;

    for i in input.split("\n") {
        match i.trim().parse::<i32>() {
            Ok(x) => total += x,
            Err(_) => {
                if total > max { 
                    max = total;
                }
                total = 0;
            }
        }
    }

    max
}

//================================================================================================

fn find_elves_with_most_calories(input: String) -> i32 {
    let mut total = 0;
    let mut top_three: Vec<i32> = vec!(0, 0, 0);

    for i in input.split("\n") {
        match i.trim().parse::<i32>() {
            Ok(x) => total += x,
            Err(_) => {
                for (index, value) in top_three.iter().enumerate() {
                    if value < &total {
                        top_three.insert(index, total);
                        top_three.pop();
                        break;
                    }
                }

                total = 0;
            }
        }
    }

    top_three.iter().sum()
}
