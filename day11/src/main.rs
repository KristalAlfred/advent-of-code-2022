use regex::Regex;
use std::fs::read_to_string;

struct Monkey {
    items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test_divisible_by: Box<dyn Fn(i32) -> i32>,
}

fn main() {
    let file = read_to_string("data.txt");
    let mut monkeys: Vec<Monkey> = vec![];

    if let Ok(content) = file {
        // Parsing monkeys
        for monkey in content.split("\n\n") {
            println!("{}", monkey);
            let re = Regex::new(r"Starting items: (.*)\n.*Operation: new = old (.*)\n.*Test: divisible by (.*)\n.*monkey (\d)\n.*monkey (\d)").unwrap();
            let captures = re.captures(monkey).unwrap();

            let starting_items = captures.get(1).map_or(Vec::new(), |m| {
                m.as_str()
                    .split(", ")
                    .map(|digit| {
                        digit.parse::<i32>().unwrap()
                    })
                    .collect::<Vec<i32>>()
            });

            let operation: Box<dyn Fn(i32) -> i32> = captures.get(2).map_or(Box::new(|i| i), |m| {
                let parts = m.as_str().split_whitespace().collect::<Vec<&str>>();
                if let Ok(digit) = parts[1].parse::<i32>() {
                    match parts[0] {
                        "*" => {
                            return Box::new(move |i| i * digit);
                        }
                        "+" => {
                            return Box::new(move |i| i * digit);
                        }
                        _ => unreachable!("Malformed data"),
                    }
                } else {
                    return Box::new(|i| i * i);
                }
            });

            let if_true_pass_to = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
            let if_false_pass_to = captures.get(5).unwrap().as_str().parse::<i32>().unwrap();

            let test_divisible_by: Box<dyn Fn(i32) -> i32> = captures.get(3)
                .map_or(Box::new(|_| 0), |m| {
                    let digit = m.as_str().parse::<i32>().unwrap();
                    return Box::new(move |i| if i % digit == 0 { if_true_pass_to } else { if_false_pass_to });
                });

            monkeys.push(Monkey{ items: starting_items, operation, test_divisible_by })
        }

        // Do the rounds:
        //  1. Find the two most active monkeys
        //  2. Multiply their respective amount of inspections to get result!
    }
}