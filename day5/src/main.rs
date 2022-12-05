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
            .for_each(|chunk| {
                let crates_to_move = chunk[0].parse::<u32>().unwrap();
                let from_container = chunk[1].parse::<u32>().unwrap() - 1;
                let to_container = chunk[2].parse::<u32>().unwrap() - 1;

                println!(
                    "Moving {crates_to_move} from container {from_container} to {to_container}"
                );
                if from_container == to_container {
                    return;
                }

                for _ in 0..crates_to_move {
                    let container = containers[from_container as usize].pop();
                    if let Some(container) = container {
                        containers[to_container as usize].push(container);
                    }
                }
            });
    }
    
    for container in containers {
        println!("{:?}", container);
    }
}
