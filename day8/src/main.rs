use std::fs::read_to_string;

fn main() {
    let file = read_to_string("data.txt");

    if let Ok(contents) = file {
        let grid = convert_to_grid(contents);

        let result = get_visibility_grid(&grid).iter().flatten().fold(0, |mut acc, &bool| {
            if bool { acc += 1 }
            acc
        });
        println!("Trees visible from the outside: {result}")
    }
}

fn get_visibility_grid(grid: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let mut visibility_grid = vec![vec![true; grid.len()]; grid.len()];

    let mut maximum_scenic_score = 0;

    for (row_number, row) in grid.iter().enumerate() {
        for (column_number, &cell_value) in row.iter().enumerate() {
            let mut scenic_score = 1;
            let mut visible_in_any_direction = [true, true, true, true];

            for dir in 0..4 {
                let mut local_scenic_score = 0;
                match dir {
                    0 => {
                        // Check north
                        for position in (0..row_number).rev() {
                            local_scenic_score += 1;
                            if grid[position][column_number] >= cell_value {
                                visible_in_any_direction[0] = false;
                                break;
                            }
                        }
                        scenic_score *= local_scenic_score;
                    },
                    1 => {
                        // Check east
                        for position in column_number..grid.len() {
                            if position == column_number { continue; }
                            local_scenic_score += 1;
                            if grid[row_number][position] >= cell_value {
                                visible_in_any_direction[1] = false;
                                break;
                            }
                        }
                        scenic_score *= local_scenic_score;
                    },
                    2 => {
                        // Check south
                        for position in row_number..grid.len() {
                            if position == row_number { continue; }
                            local_scenic_score += 1;
                            if grid[position][column_number] >= cell_value {
                                visible_in_any_direction[2] = false;
                                break;
                            }
                        }
                        scenic_score *= local_scenic_score;
                    },
                    3 => {
                        // Check west
                        for position in (0..column_number).rev() {
                            local_scenic_score += 1;
                            if grid[row_number][position] >= cell_value {
                                visible_in_any_direction[3] = false;
                                break;
                            }
                        }
                        scenic_score *= local_scenic_score;
                    }
                    _ => unreachable!()
                }
            }
            visibility_grid[column_number][row_number] = visible_in_any_direction.iter().any(|&b| b);
            maximum_scenic_score = std::cmp::max(scenic_score, maximum_scenic_score);
        }
    }
    println!("Maximum scenic score: {maximum_scenic_score}");

    visibility_grid
}

fn convert_to_grid(data: String) -> Vec<Vec<u32>> {
    let mut output: Vec<Vec<u32>> = vec![vec![]];

    let mut row = 0;

    for char in data.chars() {
        if char == '\n' {
            output.push(vec![]);
            row += 1;
        } else {
            if let Some(digit) = char.to_digit(10) {
                output[row].push(digit);
            } else {
                panic!("Malformed data!");
            }
        }
    }
    output
}
