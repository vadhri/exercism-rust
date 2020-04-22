use std::collections::HashMap;
use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

#[derive(Debug, PartialOrd, PartialEq)]
pub enum PokerHand {
    Unknown,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind
}

pub fn check_type(
    set_to_value: &mut HashMap<char, Vec<u32>>,
    value_no_of_repeats: &mut HashMap<u32, u32>) -> PokerHand {

    let is_sequential = || -> bool {
        let mut value_vector = Vec::new();

        for value in set_to_value.clone().values_mut() {
            value_vector.append(value);
        }

        value_vector.sort();

        if value_vector.contains(&2) && value_vector.contains(&14) {
            value_vector.pop();
            value_vector.push(1);
            value_vector.sort();
        }

        value_vector
            .windows(2)
            .filter(|value| {
                value[1] - value[0] != 1
            })
            .count() == 0
    };

    let repeats = || -> Option<u32> {
        match value_no_of_repeats.values().max() {
            Some(x) => Some(*x),
            None => None
        }
    };

    let dedup_values = || -> Vec<u32> {
        let mut value_vector = Vec::new();

        for value in value_no_of_repeats.values() {
            value_vector.push(*value);
        }

        value_vector.sort();
        value_vector
    };

    let mut pattern: PokerHand = PokerHand::HighCard;
    let sequence_check = is_sequential();
    let repeats_check = match repeats() {
        Some(x) => x,
        None => 0
    };

    let duplicates_vector = dedup_values();
    let two_pair_check = duplicates_vector == vec![1,2,2];
    let one_pair_check = duplicates_vector.iter().filter(|pair| **pair == 2).count() == 1;

    match set_to_value.keys().len() {
        1 => {
            // FiveOfAKind && Flush && FourOfAKind &&  StraightFlush - straight flush
            if sequence_check == true {
                pattern = PokerHand::StraightFlush;
            } else if repeats_check > 1  {
                if repeats_check == 4 {
                    pattern = PokerHand::FourOfAKind;
                }
            } else {
                pattern = PokerHand::Flush;
            }
        },
        2 => {
            // FiveOfAKind ( 4 + wildcard) && FourOfAKind && FullHouse (d,d,d,s,s)
            if repeats_check > 1  {
                if repeats_check == 4 {
                    pattern = PokerHand::FourOfAKind;
                } else if repeats_check == 3 {
                    if duplicates_vector.len() == 2 {
                        pattern = PokerHand::FullHouse;
                    } else {
                        pattern = PokerHand::ThreeOfAKind;
                    }
                }
            }
        },
        3 => {
            // FullHouse && ThreeOfAKind (d,d,d,s,h) && OnePair (s,s,d,d,h) && TwoPair (s,s,d,d,h)
            if sequence_check == true {
                pattern = PokerHand::Straight;
            } else if repeats_check > 1  {
                if repeats_check == 4 {
                    pattern = PokerHand::FourOfAKind;
                } else if repeats_check == 3 {
                    if duplicates_vector.len() == 2 {
                        pattern = PokerHand::FullHouse;
                    } else {
                        pattern = PokerHand::ThreeOfAKind;
                    }
                } else if two_pair_check == true {
                    pattern = PokerHand::TwoPair;
                } else if one_pair_check == true {
                    pattern = PokerHand::OnePair;
                }
            }
        },
        4 => {
            // Straight && OnePair of a kind (s,s,d,c,h) && HighCard.
            if sequence_check == true {
                pattern = PokerHand::Straight;
            } else if repeats_check > 1  {
                if repeats_check == 4 {
                    pattern = PokerHand::FourOfAKind;
                } else if repeats_check == 3 {
                    if duplicates_vector.len() == 2 {
                        pattern = PokerHand::FullHouse;
                    } else {
                        pattern = PokerHand::ThreeOfAKind;
                    }
                } else if two_pair_check == true {
                    pattern = PokerHand::TwoPair;
                } else if one_pair_check == true {
                    pattern = PokerHand::OnePair;
                }
            } else if two_pair_check == true {
                pattern = PokerHand::TwoPair;
            } else if one_pair_check == true {
                pattern = PokerHand::OnePair;
            }
        },
        _ => {}
    }
    pattern
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut card_set_compare = HashMap::new();
    card_set_compare.insert('S', 4);
    card_set_compare.insert('D', 3);
    card_set_compare.insert('H', 2);
    card_set_compare.insert('C', 1);

    let convert_chars_to_values = |s: String| -> u32 {
        match s.as_str() {
            "A" => 14,
            "K" => 13,
            "Q" => 12,
            "J" => 11,
            rest => rest.parse::<u32>().unwrap()
        }
    };

    let mut output = Vec::new();
    let mut output_hand = PokerHand::Unknown;
    let mut output_highcard = 0;
    let mut output_highcard_set = Vec::new();
    let mut output_number_to_set: HashMap<u32, Vec<char>> = HashMap::new();

    for a in hands {
        let mut set_to_number: HashMap<char, Vec<u32>> = HashMap::new();
        let mut number_to_set: HashMap<u32, Vec<char>> = HashMap::new();
        let mut number_to_repeats: HashMap<u32, u32> = HashMap::new();

        for card in a.split(" ").collect::<Vec<_>>() {
            let vector = card.chars().collect::<Vec<_>>();
            let set = vector.last().unwrap();
            let card_value = convert_chars_to_values(vector[0..vector.len()-1].to_vec().iter().collect::<String>());

            match set_to_number.contains_key(set) {
                true => {
                    let value = set_to_number.get_mut(set).unwrap();
                    value.push(card_value);
                    value.sort();

                    match number_to_set.get_mut(&card_value) {
                        Some(x) => {
                            x.push(*set);
                            x.sort();
                        }
                        None => {
                            number_to_set.insert(card_value, vec![*set]);
                        }
                    }

                    match number_to_repeats.get_mut(&card_value) {
                        Some(x) => {
                            *x += 1;
                        }
                        None => {
                            number_to_repeats.insert(card_value, 1);
                        }
                    }
                },
                false => {
                    set_to_number.insert(*set, vec![card_value]);

                    match number_to_set.get_mut(&card_value) {
                        Some(x) => {
                            x.push(*set);
                            x.sort();
                        }
                        None => {
                            number_to_set.insert(card_value, vec![*set]);
                        }
                    }

                    match number_to_repeats.get_mut(&card_value) {
                        Some(x) => {
                            *x += 1;
                        }
                        None => {
                            number_to_repeats.insert(card_value, 1);
                        }
                    }
                }
            }
        }

        let hand = check_type(&mut set_to_number, &mut number_to_repeats);

        if output_hand < hand {
            output.clear();
            output.push(*a);
            output_hand = hand;
            output_highcard = *number_to_repeats.keys().max().unwrap();

            output_highcard_set.clear();

            let _val = set_to_number.iter().map(|(k, v)| {
                if v.contains(&output_highcard) {
                    output_highcard_set.push(k.clone());
                }
            }).collect::<Vec<_>>();

            output_number_to_set = number_to_set.clone();
        } else if output_hand == hand {
            let highcard = *number_to_repeats.keys().max().unwrap();

            if output_hand == PokerHand::HighCard && output_highcard < highcard  {
                if output_highcard < highcard {
                    output_highcard = highcard;
                    output.clear();
                    output.push(*a);
                    output_hand = hand;
                    // output_set_to_number = set_to_number.clone();
                    output_number_to_set = number_to_set.clone();
                } else if output_highcard == highcard {
                    output.push(*a);
                    output_hand = hand;
                }
            } else  {
                let mut number_to_set_pair = number_to_set.iter().collect::<Vec<_>>();
                let mut output_number_to_set_pair = output_number_to_set.iter().collect::<Vec<_>>();

                number_to_set_pair.sort_by(|a, b| {
                    if a.1.len() > b.1.len() {
                        Ordering::Less
                    } else if a.1.len() == b.1.len() {
                        if a.0 > b.0 {
                            Ordering::Less
                        } else if a.0 < b.0 {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    } else {
                        Ordering::Greater
                    }
                });

                output_number_to_set_pair.sort_by(|a, b| {
                    if a.1.len() > b.1.len() {
                        Ordering::Less
                    } else if a.1.len() == b.1.len() {
                        if a.0 > b.0 {
                            Ordering::Less
                        } else if a.0 < b.0 {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    } else {
                        Ordering::Greater
                    }
                });

                let mut number_to_set_pair_keys: Vec<u32> = number_to_set_pair.iter().map(|pair| *pair.0).collect::<Vec<_>>();
                let mut output_number_to_set_pair_keys: Vec<u32> = output_number_to_set_pair.iter().map(|pair| *pair.0).collect::<Vec<_>>();

                let number_to_set_pair_keys_correction_for_ace = number_to_set_pair_keys.contains(&2) && number_to_set_pair_keys.contains(&14);
                let output_number_to_set_pair_keys_correction_for_ace = number_to_set_pair_keys.contains(&2) && number_to_set_pair_keys.contains(&14);

                if number_to_set_pair_keys_correction_for_ace {
                    number_to_set_pair_keys = number_to_set_pair_keys.iter().map(|item| {
                        if *item == 14 {
                            1
                        } else {
                            *item
                        }
                    }).collect::<Vec<_>>();
                }

                if output_number_to_set_pair_keys_correction_for_ace {
                    output_number_to_set_pair_keys = output_number_to_set_pair_keys.iter().map(|item| {
                        if *item == 14 {
                            1
                        } else {
                            *item
                        }
                    }).collect::<Vec<_>>();
                }
                if number_to_set_pair_keys > output_number_to_set_pair_keys {
                    output.clear();
                    output.push(*a);
                    output_hand = hand;
                } else if number_to_set_pair_keys == output_number_to_set_pair_keys {
                    output.push(*a);
                    output_hand = hand;
                }
            }
        }
    }

    Some(output)
}
