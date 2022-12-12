use regex::Regex;
use std::fs::read_to_string;

struct Monkey {
    items: Vec<i32>,
    operation: fn(i32) -> i32,
    test: fn(i32) -> bool,
}

fn main() {
    let file = read_to_string("data.txt");
    let monkeys: Vec<Monkey> = vec![];

    if let Ok(content) = file {

        // Parsing monkeys
        for monkey in content.split("\n\n") {
            println!("NEW MONKEY!");
            println!("{}", monkey);

            let re = Regex::new(r"(Starting items:) (.*)\n.*(Operation: new = old) (.*)\n.*(Test: divisible by) (.*)(\n.*monkey )(\d)(\n.*monkey )(\d)").unwrap();

            let captures = re.captures(monkey).unwrap();

            let starting_items = captures.get(1).map_or(Vec::new(), |m| {
                m.as_str()
                    .split(", ")
                    .map(|digit| digit.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            });

            let operation: fn(i32) -> i32 = captures.get(2).map_or(
                |i| i,
                |m| {
                    let parts = m.as_str().split_whitespace().collect::<Vec<&str>>();
                    match parts[0] {
                        "*" => {
                            // Return functor(?) that multiplies a value with parts[1]
                        },
                        "+" => {
                            // Returns functor(?) that adds a value with parts[1]
                        }
                    }

                },
            );

            let test_divisble_by = captures.get(3).unwrap();
            let if_true_pass_to = captures.get(4).unwrap();
            let if_false_pass_to = captures.get(5).unwrap();
        }

        // Do the rounds:
        //  1. Find the two most active monkeys
        //  2. Multiply their respective amount of inspections to get result!
    }
}

fn multiply(factor1: i32, factor2: i32) -> i32 {
    factor1 * factor2
}

fn add(operand1: i32, operand2: i32) -> i32 {
    operand1 + operand2
}