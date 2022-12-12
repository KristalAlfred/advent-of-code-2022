use regex::Regex;
use std::{fs::read_to_string, cell::RefCell, collections::VecDeque};

struct Monkey {
    items: RefCell<VecDeque<i32>>,
    operation: Box<dyn Fn(i32) -> i32>,
    test_divisible_by: Box<dyn Fn(i32) -> i32>,
    inspected_items: u32
}

impl Monkey {
    fn handle_item(&self) -> i32 {
        if let Some(item) = self.items.borrow_mut().pop_front() {
            return (self.test_divisible_by)((self.operation)(item) / 3);
        } else {
            panic!("Tried handling an item that doesn't exist.");
        }
    }
}

fn main() {
    let file = read_to_string("data.txt");
    
    if let Ok(content) = file {
        // Parsing monkeys
        let mut monkeys: Vec<Monkey> = parse_monkeys(content);

        // Do the rounds:
        for _ in 0..20 {
            for monkey in &mut monkeys {

                let mut len = 0;

                {
                    len = monkey.items.borrow().len().clone();
                }

                for _ in 0..len {
                    monkey.handle_item();
                    monkey.inspected_items += 1;
                }
            }
        }

        for monkey in monkeys {
            println!("{}", monkey.inspected_items);
        }
        //  1. Find the two most active monkeys
        //  2. Multiply their respective amount of inspections to get result!
    }
}

fn parse_monkeys(unparsed_data: String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];

    for monkey in unparsed_data.split("\n\n") {
        println!("{}", monkey);
        let re = Regex::new(r"Starting items: (.*)\n.*Operation: new = old (.*)\n.*Test: divisible by (.*)\n.*monkey (\d)\n.*monkey (\d)").unwrap();
        let captures = re.captures(monkey).unwrap();

        let starting_items = captures.get(1).map_or(VecDeque::new(), |m| {
            m.as_str()
                .split(", ")
                .map(|digit| {
                    digit.parse::<i32>().unwrap()
                })
                .collect::<VecDeque<i32>>()
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

        monkeys.push(Monkey{ items: RefCell::new(starting_items), operation, test_divisible_by, inspected_items: 0 })
    }

    monkeys
}