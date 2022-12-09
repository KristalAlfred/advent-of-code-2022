use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let file = read_to_string("data.txt");

    let mut visited_positions: HashSet<(u32, u32)> = HashSet::new();
    let starting_pos = (u32::MAX / 2, u32::MAX / 2);
    let mut head_pos = starting_pos;
    let mut tail_pos = starting_pos;
    visited_positions.insert(starting_pos);

    if let Ok(contents) = file {
        for ele in contents.split('\n') {
            let vec = ele.split_whitespace().collect::<Vec<_>>();
            let steps = vec[1].parse::<u32>().unwrap();
            match vec[0] {
                "R" => {
                    for _ in 0..steps {
                        let (mut head_x, head_y) = head_pos;
                        head_x += 1;
                        head_pos = (head_x, head_y);
                        tail_pos = update_tail_pos(head_pos, tail_pos);
                        visited_positions.insert(tail_pos);
                    }
                }
                "L" => {
                    for _ in 0..steps {
                        let (mut head_x, head_y) = head_pos;
                        head_x -= 1;
                        head_pos = (head_x, head_y);
                        tail_pos = update_tail_pos(head_pos, tail_pos);
                        visited_positions.insert(tail_pos);
                    }
                }
                "U" => {
                    for _ in 0..steps {
                        let (head_x, mut head_y) = head_pos;
                        head_y += 1;
                        head_pos = (head_x, head_y);
                        tail_pos = update_tail_pos(head_pos, tail_pos);
                        visited_positions.insert(tail_pos);
                    }
                }
                "D" => {
                    for _ in 0..steps {
                        let (head_x, mut head_y) = head_pos;
                        head_y -= 1;
                        head_pos = (head_x, head_y);
                        tail_pos = update_tail_pos(head_pos, tail_pos);
                        visited_positions.insert(tail_pos);
                    }
                }
                _ => unreachable!("Malformed data!"),
            }
        }
    }
    println!(
        "Number of positions visited by tail: {}",
        visited_positions.len()
    );
}

fn update_tail_pos(head_position: (u32, u32), tail_position: (u32, u32)) -> (u32, u32) {
    // Move 1 step closer on both x coordinate and y coordinate, but only if we are more than
    // 1 step away!
    let (head_x, head_y) = head_position;
    let (tail_x, tail_y) = tail_position;

    if head_x.abs_diff(tail_x) <= 1 && head_y.abs_diff(tail_y) <= 1 {
        return tail_position;
    }

    let mut new_tail_position = (0u32, 0u32);
    match head_x.cmp(&tail_x) {
        std::cmp::Ordering::Equal => {
            new_tail_position.0 = tail_x;
        }
        std::cmp::Ordering::Greater => {
            new_tail_position.0 = tail_x + 1;
        }
        std::cmp::Ordering::Less => {
            new_tail_position.0 = tail_x - 1;
        }
    }

    match head_y.cmp(&tail_y) {
        std::cmp::Ordering::Equal => {
            new_tail_position.1 = tail_y;
        }
        std::cmp::Ordering::Greater => {
            new_tail_position.1 = tail_y + 1;
        }
        std::cmp::Ordering::Less => {
            new_tail_position.1 = tail_y - 1;
        }
    }

    new_tail_position
}
