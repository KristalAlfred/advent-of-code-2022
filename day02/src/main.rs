fn main() {
    let file = std::fs::read_to_string("data.txt");
    if let Ok(contents) = file {
        let result: u32 = contents
            .split("\n")
            .map(|str| {
                str.chars()
                    .filter(|&ch| ch != ' ')
                    .map(|ch| ch as u32)
                    .collect::<Vec<u32>>()
                    .chunks(2)
                    .fold(0, {
                        //part1
                        part2
                    })
            })
            .sum();
        println!("{:?}", result);
    }
}

fn part1(mut acc: u32, data: &[u32]) -> u32 {
    // Use values 0-2 instead of A-C/X-Z
    let we = data[1] - 88;
    let them = data[0] - 65;
    if (them + 1) % 3 == we {
        // We win!
        acc += 6;
    } else if (we + 1) % 3 == them {
        // We lose, nothing happens :(
    } else {
        // Draw
        acc += 3;
    }
    acc += we + 1;
    acc
}

fn part2(mut acc: u32, data: &[u32]) -> u32 {
    // Use values 0-2 instead of A-C/X-Z
    let result = data[1] - 88;
    let them = data[0] - 65;
    let mut we: i32 = them as i32 - 1 + result as i32;
    we = if we < 0 { we + 3 } else { we % 3 };

    // Add winning score & compensate for using range 0-2 instead of 1-3 in last step.
    acc += 1 + result * 3 + we as u32;
    acc
}