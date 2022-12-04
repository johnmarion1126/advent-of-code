fn find_total_overlapping_pairs(input: &str) -> i32 {
    let mut total = 0;

    for i in input.lines() {
        let sections = i.trim().split(",").collect::<Vec<&str>>();

        let first_section_chars = sections[0].split("-").collect::<Vec<&str>>();
        let second_section_chars = sections[1].split("-").collect::<Vec<&str>>();

        let first_section_start: i32 = first_section_chars[0].parse().unwrap();
        let first_section_end: i32 = first_section_chars[1].parse().unwrap();

        let second_section_start: i32 = second_section_chars[0].parse().unwrap();
        let second_section_end: i32 = second_section_chars[1].parse().unwrap();

        if (first_section_start <= second_section_start && first_section_end >= second_section_end)
            || (first_section_start >= second_section_start
                && first_section_end <= second_section_end)
        {
            total += 1;
        }
    }

    total
}

//================================================================================================
