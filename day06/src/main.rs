use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let data = read_to_string("data.txt");

    let start_of_message_counter = 14; // 4 for part 1 solution.

    let mut result = 0;
    if let Ok(content) = data {
        for (i, win) in content.as_bytes()
                                            .iter()
                                            .as_slice()
                                            .windows(start_of_message_counter)
                                            .enumerate() {
            {
                let mut map = HashSet::new();
                let is_unique_sequence = win.into_iter().all(move |element| map.insert(element));
                if is_unique_sequence {
                    // 0-indexed and the challenge is asking for the ending index, i is the starting one.
                    result = i + start_of_message_counter;
                    break;
                }
            }
        }
    }

    println!("{result}");
}
