use std::{
    io::{BufReader, self, BufRead}, 
    fs::{File, read_to_string}
};

fn main() {
    // Part 1
    let mut result_part1 = 0;
    for line in read_lines("data.txt") {
        if let Ok(entry) = line {
            let other_half = &entry[entry.len() / 2..];
            for char in entry.chars() {
                if other_half.contains(char) {
                    result_part1 += get_char_value(char);
                    break;
                }
            }
        }
    }

    // Part 2
    let file = read_to_string("data.txt").unwrap();
    let result_part2 = file.split("\n")
        .collect::<Vec<_>>()
        .chunks(3)
        .fold(0, |mut acc, lines| {
            for char in lines[0].chars() {
                if lines[1].contains(char) && lines[2].contains(char) {
                    acc += get_char_value(char);
                    return acc;
                }
            }
            unreachable!();
        });

    // Results
    println!("{}", result_part1);
    println!("{}", result_part2);
}

fn get_char_value (char: char) -> u32 {
    // a-z = 1-26
    // A-Z = 27-52
    match char {
        'a'..='z' => char as u32 - 96,
        'A'..='Z' => char as u32 - 38,
        _ => 0,
    }
}

fn read_lines(filename: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
