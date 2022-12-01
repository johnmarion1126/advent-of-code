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
