use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let file = read_to_string("data.txt");

    let mut visited_positions: HashSet<(u32, u32)> = HashSet::new();
    let starting_pos = (u32::MAX / 2, u32::MAX / 2);
    visited_positions.insert(starting_pos);

    // Change rope length here to solve the different parts!
    let rope_len = 10;
    let mut rope = vec![starting_pos; rope_len];

    if let Ok(contents) = file {
        for ele in contents.split('\n') {
            let vec = ele.split_whitespace().collect::<Vec<_>>();
            let steps = vec[1].parse::<u32>().unwrap();

            for _ in 0..steps {
                let (head_x, head_y) = &mut rope[0];
                match vec[0] {
                    "R" => *head_x += 1,
                    "L" => *head_x -= 1,
                    "U" => *head_y += 1,
                    "D" => *head_y -= 1,
                    _ => unreachable!("Malformed data!"),
                }
                rope[0] = (*head_x, *head_y);

                visited_positions.insert(update_rope_pos(&mut rope));
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

fn update_rope_pos(rope: &mut Vec<(u32, u32)>) -> (u32, u32) {

    for i in 1..rope.len() {
        rope[i] = update_tail_pos(rope[i - 1], rope[i]);
    }

    rope[rope.len() - 1]
}