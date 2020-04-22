#[derive(Debug, PartialEq, PartialOrd)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: std::fmt::Debug +  std::clone::Clone +  PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut output: Comparison = Comparison::Unequal;

    if _first_list == _second_list {
        output = Comparison::Equal;
    } else {
        if _first_list.len() == 0 {
            output = Comparison::Sublist;
        } else if _second_list.len() == 0 {
            output = Comparison::Superlist;
        } else {
            match _first_list.len() > _second_list.len() {
                true =>  {
                    match _first_list.windows(_second_list.len()).position(|window| window == _second_list) {
                        Some(_window) => {
                            output = Comparison::Superlist;
                        },
                        None => {
                            output = Comparison::Unequal;
                        }
                    }
                },
                false => {
                    match _second_list.windows(_first_list.len()).position(|window| window == _first_list) {
                        Some(_window) => {
                            output = Comparison::Sublist;
                        },
                        None => {
                            output = Comparison::Unequal;
                        }
                    }
                }
            };
        }
    }

    output
}
