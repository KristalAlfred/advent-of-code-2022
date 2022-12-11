use std::fs::read_to_string;

fn main() {
    let file = read_to_string("data.txt");

    if let Ok(contents) = file {
        let instructions: Vec<Vec<&str>> = contents
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .collect();

        let signal_strength = get_signal_strength(&instructions);
        let crt = draw_crt(&instructions);
        println!("Signal strength: {signal_strength}");

        for row in crt {
            for ch in row {
                print!("{ch}");
            }
            println!("");
        }
    }
}

fn get_signal_strength(instructions: &Vec<Vec<&str>>) -> i32 {
    let mut cycle_count = 0;
    let mut register = 1;
    let mut signal_strength = 0;

    for instruction in instructions {
        match instruction[0] {
            "addx" => {
                for _ in 0..2 {
                    cycle_count += 1;
                    if cycle_count % 40 == 20 {
                        signal_strength += register * cycle_count;
                    }
                }

                register += instruction[1].parse::<i32>().unwrap();
            }
            "noop" => {
                cycle_count += 1;
                if cycle_count % 40 == 20 {
                    signal_strength += register * cycle_count;
                }
            }
            _ => unreachable!("Malformed data"),
        }
    }

    signal_strength
}

fn draw_crt(instructions: &Vec<Vec<&str>>) -> Vec<Vec<char>> {
    let mut cycle_count = 0;
    let mut crt_row = 0;
    let mut sprite_position: i32 = 1;

    let mut crt = vec![vec![' '; 40]; 6];

    for instruction in instructions {
        match instruction[0] {
            "addx" => {
                for _ in 0..2 {
                    println!("Going.. crt_row={crt_row}, cycle_count={cycle_count}");
                    crt[crt_row][cycle_count] = if ((cycle_count as i32-1)..=(cycle_count as i32+1)).contains(&sprite_position) {
                        '#'
                    } else {
                        '.'
                    };
                    cycle_count += 1;
                    crt_row += if cycle_count == 40 { 1 } else { 0 };
                    cycle_count = cycle_count % 40;
                }

                sprite_position += instruction[1].parse::<i32>().unwrap();
            }
            "noop" => {
                crt[crt_row][cycle_count] = if ((cycle_count as i32-1)..(cycle_count as i32+1)).contains(&sprite_position) {
                    '#'
                } else {
                    '.'
                };
                cycle_count += 1;
                crt_row += if cycle_count == 40 { 1 } else { 0 };
                cycle_count = cycle_count % 40;
            }
            _ => unreachable!("Malformed data"),
        }
    }

    crt
}
