use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn part1() {
    let threshold = 100_000;
    let mut directories_beneath_threshold_size = 0;
    let mut level_counts = vec![];

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
                                        // cd .. — change current Directory to parent Directory
                                        if dir.contains('.') {
                                            let size = level_counts.pop().unwrap();
                                            if size <= threshold {
                                                directories_beneath_threshold_size += size;
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
                'd' => { /* Do nothing for "dir ..." lines */ },
                _ => unreachable!("Malformed data")
            }
        }
    }
    println!("Directories under or equal to threshold: {directories_beneath_threshold_size}");
}

fn part2() {
    let total_space_available = 70_000_000;
    let space_needed_for_upgrade = 30_000_000;
    let mut level_counts = vec![];
    let mut total_file_size = 0;
    let mut unused_space = 0;
    let mut closest_dir_size: u64 = u32::MAX as u64;

    let data = read_to_string("data.txt");
    if let Ok(contents) = data {
        for line in contents.split('\n') {
            match line.chars().peekable().peek().unwrap() {
                '0'..='9' => {
                    // Parse the file size and add to current Directory size.
                    match line.split_whitespace().map(|str| str.parse::<u64>()).next() {
                        Some(number) => {
                            total_file_size += number.unwrap();
                        },
                        None => {}
                    }
                },
                _ => { /* Ignore everything else */ }
            }
        }
    }

    unused_space = total_space_available - total_file_size;

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
                                        // cd .. — change current Directory to parent Directory
                                        if dir.contains('.') {
                                            let size = level_counts.pop().unwrap();
                                            let is_closer = unused_space + size < unused_space + closest_dir_size && 
                                                unused_space + size > space_needed_for_upgrade;

                                            if is_closer {
                                                closest_dir_size = size;
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
        }
    }
    println!("Smallest dir to delete: {closest_dir_size}");
}