use regex::Regex;
use std::{fs::read_to_string, cell::RefCell, collections::VecDeque};

struct Monkey {
    items: RefCell<VecDeque<u128>>,
    operation: Box<dyn Fn(&mut u128) -> u128>,
    test_divisible_by: Box<dyn Fn(u128) -> u128>,
    div: u32
}

impl Monkey {
    fn handle_item(&self) -> (u128, u128) {
        if let Some(mut item) = self.items.borrow_mut().pop_front() {
            let item = (self.operation)(&mut item); // divide by three for part 1!
            return ((self.test_divisible_by)(item), item);
        } else {
            panic!("Tried handling an item that doesn't exist.");
        }
    }
}

fn main() {
    let file = read_to_string("data.txt");
    
    if let Ok(content) = file {
        // Parsing monkeys
        let monkeys: Vec<Monkey> = parse_monkeys(content);

        // Do the rounds:
        let rounds = 10000;
        println!("Monkey business for {rounds} rounds is: {}", get_monkey_business(&monkeys, rounds));
    }
}

fn parse_monkeys(unparsed_data: String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];

    for monkey in unparsed_data.split("\n\n") {
        let re = Regex::new(r"Starting items: (.*)\n.*Operation: new = old (.*)\n.*Test: divisible by (.*)\n.*monkey (\d)\n.*monkey (\d)").unwrap();
        let captures = re.captures(monkey).unwrap();

        let starting_items = captures.get(1).map_or(VecDeque::new(), |m| {
            m.as_str()
                .split(", ")
                .map(|digit| {
                    digit.parse::<u128>().unwrap()
                })
                .collect::<VecDeque<u128>>()
        });

        let operation: Box<dyn Fn(&mut u128) -> u128> = captures.get(2).map_or(Box::new(|i| i.clone()), |m| {
            let parts = m.as_str().split_whitespace().collect::<Vec<&str>>();
            if let Ok(digit) = parts[1].parse::<u128>() {
                match parts[0] {
                    "*" => {
                        return Box::new(move |i| { 
                            *i = *i * digit; 
                            i.clone() 
                        });
                    }
                    "+" => {
                        return Box::new(move |i| { 
                            *i = *i + digit; 
                            i.clone() 
                        });
                    }
                    _ => unreachable!("Malformed data"),
                }
            } else {
                return Box::new(|i| {
                    *i = (*i) * (*i); 
                    i.clone()
                });
            }
        });

        let if_true_pass_to = captures.get(4).unwrap().as_str().parse::<u128>().unwrap();
        let if_false_pass_to = captures.get(5).unwrap().as_str().parse::<u128>().unwrap();

        let div = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();

        let test_divisible_by: Box<dyn Fn(u128) -> u128> = captures.get(3)
            .map_or(Box::new(|_| 0), |m| {
                let digit = m.as_str().parse::<u128>().unwrap();
                return Box::new(move |i| if i % digit == 0 { if_true_pass_to } else { if_false_pass_to });
            });

        monkeys.push(Monkey{ items: RefCell::new(starting_items), operation, test_divisible_by, div })
    }

    monkeys
}

fn get_monkey_business(monkeys: &Vec<Monkey>, rounds: u32) -> u128 {

    let modulo: u32 = monkeys.iter().map(|m| m.div).product();

    let mut inspection_count = vec![0; monkeys.len()];
        for round in 0..rounds {
            println!("On round {round}");
            for (i, monkey) in monkeys.iter().enumerate() {
                let len;
                {
                    len = monkey.items.borrow().len().clone();
                }
                for _ in 0..len {
                    let (destination, item) = monkey.handle_item();
                    {
                        monkeys[destination as usize].items.borrow_mut().push_back(item % modulo as u128);
                    }
                    inspection_count[i] += 1;
                }
            }
        }

        let two_highest = get_two_highest(inspection_count);
        two_highest.iter().fold(1, |acc, value| acc * *value)
}

fn get_two_highest(vec: Vec<u128>) -> [u128; 2] {
    let mut vec = vec.clone();
    let mut two_highest = [0; 2];
    for i in 0..2 {
        let mut highest_index = 0;
        let mut highest = 0;
        for (idx, count) in vec.iter().enumerate() {
            if *count > highest {
                highest = count.clone();
                highest_index = idx;
            }
        }
        two_highest[i] = highest;
        vec.remove(highest_index);
    }
    two_highest
}