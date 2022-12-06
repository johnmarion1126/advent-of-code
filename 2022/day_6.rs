fn get_marker_position(input: &str, window_size: usize) -> usize {
    let mut map_window: HashSet<char> = HashSet::new();
    let mut window: VecDeque<char> = VecDeque::new();
    let input_chars: Vec<char> = input.chars().collect();

    let mut index = 0;
    while index < input.len() {
        let key = input_chars[index];

        if map_window.contains(&key) {
            map_window.remove(&key);

            while let Some(x) = window.pop_front() {
                map_window.remove(&x);
                if x == key {
                    break;
                }
            }
        }

        window.push_back(key);
        map_window.insert(key);

        if window.len() == window_size {
            return index + 1;
        }

        index += 1;
    }

    index
}
