use std::fs::read_to_string;

fn main() {
    let file = read_to_string("data.txt");

    let mut cycle_count = 0;
    let mut register = 1;
    let mut signal_strength = 0;
    if let Ok(contents) = file {
        for line in contents.lines() {
            let word = line.split_whitespace().collect::<Vec<_>>();
            match word[0] {
                "addx" => {
                    for _ in 0..2 {
                        cycle_count += 1;
                        if cycle_count % 40 == 20 {
                            signal_strength += register * cycle_count;
                        }
                    }

                    register += word[1].parse::<i32>().unwrap();
                },
                "noop" => {
                    cycle_count += 1;
                    if cycle_count % 40 == 20 {
                        signal_strength += register * cycle_count;
                    }
                },
                _ => unreachable!("Malformed data")
            } 
        }
    }
    
    println!("Signal strength: {signal_strength}");
}

