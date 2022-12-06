use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let data = read_to_string("data.txt");

    let start_of_message_counter = 14;

    let mut result = 0;
    if let Ok(content) = data {
        for (i, win) in content.as_bytes()
                                            .iter()
                                            .as_slice()
                                            .windows(start_of_message_counter)
                                            .enumerate() {
            {
                let mut map = HashSet::new();
                let is_unique = win.into_iter().all(move |element| map.insert(element));
                if is_unique {
                    // 0-indexed and the challenge is asking for the ending index, i is the starting one.
                    result = i + start_of_message_counter;
                    break;
                }
            }
        }
    }

    println!("{result}");
}
