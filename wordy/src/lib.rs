pub struct WordProblem;

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Unknown,
    None
}

pub const MAX: i32 = i32::max_value(); // 2_147_483_647i32

pub fn answer(command: &str) -> Option<i32> {
    let filtered_command = command.replace("?", "")
        .replace("!", "")
        .replace("What", "")
        .replace("is ", " ")
        .replace("by", "")
        .replace("the", "")
        .replace("power", "")
        .replace("th", " ")
        .replace("nd", " ")
        .replace("rd", "")
        .replace("to", "");

    let mut op1 = MAX;
    let mut op2 = MAX;
    let mut result = 0;
    let mut operation: Op = Op::None;
    
    let check_operation = |op1: &mut i32, op2: &mut i32, operation: &mut Op, result: &mut i32| {
        match operation {
            Op::Add => {
                if *op1 != MAX && *op2 != MAX {
                    *result = *op1 + *op2;
                    *op1 = *result;
                    *op2 = MAX;

                    *operation = Op::None;
                }
            },
            Op::Subtract => {
                if *op1 != MAX && *op2 != MAX {
                    *result = *op1 - *op2;
                    *op1 = *result;
                    *op2 = MAX;

                    *operation = Op::None;
                }
            },
            Op::Multiply => {
                if *op1 != MAX && *op2 != MAX {
                    *result = *op1 * *op2;
                    *op1 = *result;
                    *op2 = MAX;

                    *operation = Op::None;
                }
            },
            Op::Divide => {
                if *op1 != MAX && *op2 != MAX {
                    *result = *op1 / *op2;
                    *op1 = *result;
                    *op2 = MAX;

                    *operation = Op::None;
                }
            },
            Op::Power => {
                if *op1 != MAX && *op2 != MAX {
                    *result = (*op1).pow(*op2 as u32);
                    *op1 = *result;
                    *op2 = MAX;

                    *operation = Op::None;
                }
            },
            _ => {}
        }
    };

    for word in filtered_command.split_whitespace() {
        match word.parse::<i32>() {
            Ok(number) => {
                if op1 == MAX {
                    op1 = number;
                    result = op1;
                }
                else if op2 == MAX {
                    op2 = number;
                }

                check_operation(&mut op1, &mut op2, &mut operation, &mut result);
            },
            Err(x) => {
                println!("{:?} Error -> {:?}", x, word);
                if operation == Op::None && op2 == MAX && op1 != MAX {
                    match word {
                        "plus" => {
                            operation = Op::Add
                        },
                        "minus" => {
                            operation = Op::Subtract
                        },
                        "multiplied" => {
                            operation = Op::Multiply
                        },
                        "divided" => {
                            operation = Op::Divide
                        },
                        "raised" => {
                            operation = Op::Power
                        },
                        _ => {
                            operation = Op::Unknown
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    check_operation(&mut op1, &mut op2, &mut operation, &mut result);

    if result == 0 || operation != Op::None || operation == Op::None && (op1 != MAX && op2 != MAX) {
        None
    } else {
        Some(result)
    }
}
