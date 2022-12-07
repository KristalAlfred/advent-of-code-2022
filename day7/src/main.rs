use std::{fs::read_to_string, vec, rc::Rc, cell::RefCell, fmt};

struct Directory {
    name: String,
    sub_directories: Vec<Rc<RefCell<Directory>>>,
    size: u64,
}

impl Directory {
    pub fn new(name: String) -> Directory {
        Directory { name, sub_directories: vec![], size: 0 }
    }

    pub fn get_size(&self) -> u64 {
        let mut size = 0;
        size += self.size;

        if self.sub_directories.len() == 0 {
            return size;
        } else {
            for sub_directory in self.sub_directories.iter() {
                size += sub_directory.borrow().get_size();
            }
        }

        size
    }

    pub fn add_sub_directory(&mut self, dir: Rc<RefCell<Directory>>) {
        self.sub_directories.push(Rc::clone(&dir));
    }
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("name", &self.name)
         .field("size", &self.size)
         .field("length", &self.sub_directories.len())
         .finish()
    }
}

fn main() {
    let dir = parse_data();
    println!("Total size: {:?}", dir);
}

fn parse_data() -> Rc<RefCell<Directory>> {
    let root = Directory::new("/".to_string());

    let mut dir_stack = vec![Rc::new(RefCell::new(root))];

    let data = read_to_string("data.txt");
    if let Ok(contents) = data {
        for line in contents.split('\n') {
            // First char can be:
            // '$' — for a command
            // \d — for a file with size
            // 'd' — for a Directory with a name
            match line.chars().peekable().peek().unwrap() {
                '$' => {
                    // Check for potential commands:
                    let mut iter = line.split_whitespace();
                    match iter.nth(1) {
                        Some(command) => {
                            // ls — do nothing
                            if command == "ls" { continue; }
                            else {
                                match iter.next() {
                                    Some(dir) => {
                                        // cd .. — change current Directory to parent Directory
                                        if dir.contains('.') {
                                            dir_stack.pop();
                                        } else {
                                            // cd [dir_name] — create sub-Directory and set Directory to current
                                            let new_dir = Rc::new(RefCell::new(Directory::new(dir.to_string())));
                                            dir_stack.push(Rc::clone(&new_dir));
                                            {
                                                let current_dir = dir_stack.last_mut().unwrap();
                                                current_dir.borrow_mut().add_sub_directory(new_dir);
                                            }
                                        }
                                    }
                                    None => {}
                                }
                            }
                        },
                        None => {}
                    }
                },
                '0'..='9' => {
                    // Parse the file size and add to current Directory size.
                    match line.split_whitespace().map(|str| str.parse::<u64>()).next() {
                        Some(number) => {
                            println!("Parsed number: {}", number.as_ref().unwrap());
                            let current_dir = dir_stack.last_mut().unwrap();
                            current_dir.borrow_mut().size += number.unwrap();
                        },
                        None => {}
                    }
                },
                'd' => {
                    // Add to subdirectories? Or ignore?
                },
                _ => unreachable!("Malformed data")
            }

            println!("{line}");

            let current_dir = dir_stack.last_mut().unwrap();
            println!("{}", current_dir.borrow().size);
        }
    }
    dir_stack.into_iter().next().unwrap()
}