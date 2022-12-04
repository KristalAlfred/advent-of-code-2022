use std::{
    fs::{read_to_string, File},
    io::{self, BufRead, BufReader},
};

fn main() {
    let entry = read_to_string("data.txt").unwrap();
    let result = entry
        .split('\n')
        .map(|chunk| {
            chunk
                .split(&['-', ','])
                .collect::<Vec<_>>()
                .iter()
                .map(|str| str.parse::<u32>().unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .fold(0, |mut acc: i32, vec| {
            let first_range = &vec[0..2];
            let second_range = &vec[2..4];
            if first_range[0] > second_range[0] {
                if first_range[1] <= second_range[1] {
                    acc += 1;
                }
            } else if first_range[0] < second_range[0] {
                if first_range[1] >= second_range[1] {
                    acc += 1;
                }
            } else {
                acc += 1;
            }
            
            acc
        });
    println!("Result: {result}");
}
