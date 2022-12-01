use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    let calories = parse_calories("input.txt");
    println!("Single most calories: {:?}", top_three(&calories).iter().max().unwrap());
    println!("Top three elves summed: {:?}", top_three(&calories).iter().sum::<u32>());
}

fn top_three(vec: &Vec<u32>) -> Vec<u32> {
    let mut result = vec![u32::MIN; 3];
    for value in vec {
        let i = find_smallest_index(&result);
        if value > &result[i] {
            result[i] = *value;
        }
    }

    result
}

fn find_smallest_index(vec: &Vec<u32>) -> usize {
    let mut smallest = u32::MAX;
    let mut index = usize::MIN;
    for (i, &value) in vec.iter().enumerate() {
        if value < smallest {
            smallest = value;
            index = i;
        }
    }
    index
}

fn parse_calories(filename: &str) -> Vec<u32> {
    let mut current_calories = 0;
    let mut calories: Vec<u32> = Vec::new();

    for line in read_lines(filename) {
        if let Ok(entry) = line {
            let entry = entry.trim();
            if entry.is_empty() {
                calories.push(current_calories);
                current_calories = 0;
            } else {
                current_calories += entry.parse::<u32>().unwrap();
            }
        }
    }

    calories
}

fn read_lines(filename: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
