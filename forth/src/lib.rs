#[macro_use] extern crate scan_fmt;

use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    methods: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            methods: HashMap::new(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut fr: ForthResult = Ok(());

        if input.chars().next().unwrap() == ':' && input.chars().last().unwrap() == ';' {
            if let Ok(a) = scan_fmt!( input, ": {} {[^,.;]}[;]", String, String) {
                let word = a.0.to_lowercase().clone();
                let expand_word: Vec<String> = a.1
                    .split_whitespace()
                    .map(|x| x.to_string().to_lowercase()).collect::<Vec<String>>();

                match word.parse::<i32>() {
                    Err(_value) => {
                        let mut existing_words_replaced = Vec::new();

                        for wd in expand_word {
                            let wd_clone = wd.clone();

                            if self.methods.contains_key(&wd_clone) {
                                for cmd in self.methods.get(&wd_clone).unwrap() {
                                    existing_words_replaced.push(cmd.clone());
                                }
                            } else {
                                existing_words_replaced.push(wd);
                            }
                        }

                        existing_words_replaced.reverse();
                        self.methods.insert(word, existing_words_replaced.clone());
                    },
                    Ok(_value) => {
                        fr = Err(Error::InvalidWord);
                    }
                }
            } else {
                fr = Err(Error::InvalidWord);
            }
        } else if input.chars().next().unwrap() == ':' && input.chars().last().unwrap() != ';' {
                fr = Err(Error::InvalidWord);
        } else {
            for word in input.split_whitespace() {
                let mut group_of_commands = Vec::new();

                group_of_commands.push(word);

                while !group_of_commands.is_empty() {
                let mut op = group_of_commands.pop().unwrap();

                if op == "+" || op == "-" || op == "*" || op == "/" {
                    if self.methods.contains_key(op) {
                        op = &self.methods.get(op).unwrap()[0];
                    }
                }

                match op.parse::<Value>() {
                    Ok(value_number) => {
                        self.stack.push(value_number);
                    }
                    Err(_reason) => match op.to_lowercase().as_str() {
                        "+" => {
                            let op1 = match self.stack.pop() {
                                Some(value) => value,
                                None => {
                                    fr = Err(Error::StackUnderflow);
                                    0
                                }
                            };
                            let op2 = match self.stack.pop() {
                                Some(value) => value,
                                None => {
                                    fr = Err(Error::StackUnderflow);
                                    0
                                }
                            };

                            if fr != Err(Error::StackUnderflow) {
                                self.stack.push(op1 + op2);
                            }
                        },
                        "-" => {
                            let op1 = match self.stack.pop() {
                                Some(value) => value,
                                None => {
                                    fr = Err(Error::StackUnderflow);
                                    0
                                }
                            };
                            let op2 = match self.stack.pop() {
                                Some(value) => value,
                                None => {
                                    fr = Err(Error::StackUnderflow);
                                    0
                                }
                            };

                            if fr != Err(Error::StackUnderflow) {
                                self.stack.push(op2 - op1);
                            }
                        },
                        "*" => {
                            let op1 = match self.stack.pop() {
                                Some(value) => value,
                                None => {
                                    fr = Err(Error::StackUnderflow);
                                    0
                                }
                            };
                            let op2 = match self.stack.pop() {
                                Some(value) => value,
                                None => {
                                    fr = Err(Error::StackUnderflow);
                                    0
                                }
                            };

                            if fr != Err(Error::StackUnderflow) {
                                self.stack.push(op2 * op1);
                            }
                        },
                        "/" => {
                            let op1 = match self.stack.pop() {
                                Some(value) => value,
                                None => {
                                    fr = Err(Error::StackUnderflow);
                                    0
                                }
                            };
                            let op2 = match self.stack.pop() {
                                Some(value) => value,
                                None => {
                                    fr = Err(Error::StackUnderflow);
                                    0
                                }
                            };

                            if fr != Err(Error::StackUnderflow) {
                                if op1 != 0 {
                                    self.stack.push(op2 / op1);
                                } else {
                                    fr = Err(Error::DivisionByZero);
                                }
                            }
                        },
                        "dup" => {
                            if !self.stack.is_empty() {
                                self.stack.push(*self.stack.last().unwrap());
                            } else {
                                fr = Err(Error::StackUnderflow);
                            }
                        },
                        "drop" => {
                            if self.stack.is_empty() {
                                fr = Err(Error::StackUnderflow);
                            } else {
                                self.stack.pop();
                            }
                        },
                        "over" => {
                            if self.stack.len() >= 2 {
                                self.stack.push(*self.stack.get(self.stack.len() - 2).unwrap());
                            } else {
                                fr = Err(Error::StackUnderflow);
                            }
                        },
                        "swap" => {
                            if self.methods.contains_key("swap"){
                                for command in self.methods.get("swap").unwrap() {
                                    group_of_commands.push(command);
                                }
                            } else {
                                if self.stack.len() >= 2 {
                                    let latest = self.stack.pop().unwrap();
                                    let penultimate = self.stack.pop().unwrap();

                                    self.stack.push(latest);
                                    self.stack.push(penultimate);
                                } else {
                                    fr = Err(Error::StackUnderflow);
                                }
                            }
                        },
                        _rest => {
                            if self.methods.contains_key(_rest){
                                for command in self.methods.get(_rest).unwrap() {
                                    group_of_commands.push(command);
                                }
                            } else {
                                fr = Err(Error::UnknownWord);
                            }
                        }
                    },
                }
            }
            }
        }

        fr
    }
}
