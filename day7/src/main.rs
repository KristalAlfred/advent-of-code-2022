use std::fs::read_to_string;

fn main() {
    let threshold = 100_000;
    let mut directories_smaller_than_threshold_size = 0;
    let mut level_counts = vec![0];

    let data = read_to_string("data.txt");
    if let Ok(contents) = data {
        for line in contents.split('\n') {
            // First char can be:
            // '$' — for a command
            // \d — for a file with size
            // 'd' — for a Directory with a name
            match line.chars().peekable().peek().unwrap() {
                '$' => {
                    // Check for potential commands:
                    let mut iter = line.split_whitespace();
                    match iter.nth(1) {
                        Some(command) => {
                            // ls — do nothing
                            if command == "ls" { continue; }
                            else {
                                match iter.next() {
                                    Some(dir) => {
                                        println!("dir: {dir}");
                                        // cd .. — change current Directory to parent Directory
                                        if dir.contains('.') {
                                            let size = level_counts.pop().unwrap();
                                            if size <= threshold {
                                                directories_smaller_than_threshold_size += size;
                                            }
                                        } else {
                                            // cd [dir_name] — advance a level in the directory tree
                                            level_counts.push(0);
                                        }
                                    }
                                    None => {}
                                }
                            }
                        },
                        None => {}
                    }
                },
                '0'..='9' => {
                    // Parse the file size and add to current Directory size.
                    match line.split_whitespace().map(|str| str.parse::<u64>()).next() {
                        Some(number) => {
                            let number = number.unwrap();
                            for level in level_counts.iter_mut() {
                                *level += number;
                            }
                        },
                        None => {}
                    }
                },
                'd' => {},
                _ => unreachable!("Malformed data")
            }
            println!("{line}");
        }
    }
    println!("Directories under or equal to threshold: {directories_smaller_than_threshold_size}");
}