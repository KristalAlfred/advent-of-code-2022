use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let mut containers = vec![
        vec!["B", "Z", "T"],
        vec!["V", "H", "T", "D", "N"],
        vec!["B", "F", "M", "D"],
        vec!["T", "J", "G", "W", "V", "Q", "L"],
        vec!["W", "D", "G", "P", "V", "F", "Q", "M"],
        vec!["V", "Z", "Q", "G", "H", "F", "S"],
        vec!["Z", "S", "N", "R", "L", "T", "C", "W"],
        vec!["Z", "H", "W", "D", "J", "N", "R", "M"],
        vec!["M", "Q", "L", "F", "D", "S"],
    ];

    let instructions = read_to_string("data.txt");
    if let Ok(instructions) = instructions {
        let regex = Regex::new(r"[a-z|\s]+").unwrap();
        regex
            .split(instructions.as_str())
            .filter(|str| !str.is_empty())
            .collect::<Vec<_>>()
            .chunks(3)
            .for_each(|instructions| {
                part1(instructions, &mut containers);
                //part2(instructions, &mut containers);
            });
    }
    print_results(&containers);
}

fn part1(instructions: &[&str], containers: &mut Vec<Vec<&str>>) {
    let containers_to_move = instructions[0].parse::<usize>().unwrap();
    let from_container = instructions[1].parse::<usize>().unwrap() - 1;
    let to_container = instructions[2].parse::<usize>().unwrap() - 1;

    if from_container == to_container {
        return;
    }

    for _ in 0..containers_to_move {
        let container = containers[from_container].pop();
        if let Some(container) = container {
            containers[to_container].push(container);
        }
    }
}

fn part2(instructions: &[&str], containers: &mut Vec<Vec<&str>>) {
    let containers_to_move = instructions[0].parse::<usize>().unwrap();
    let from_container = instructions[1].parse::<usize>().unwrap() - 1;
    let to_container = instructions[2].parse::<usize>().unwrap() - 1;

    if from_container == to_container {
        return;
    }

    let len = containers[from_container].len();
    let mut tmp = containers[from_container]
        .drain(len - containers_to_move..)
        .collect::<Vec<_>>();
    containers[to_container].append(&mut tmp);
}

fn print_results(containers: &Vec<Vec<&str>>) {
    print!("Results: ");
    for container in containers {
        print!("{}", container[container.len() - 1]);
    }
    println!("");
}
