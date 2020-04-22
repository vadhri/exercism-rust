use std::collections::HashMap;

pub fn format_line(team: String, mp: String, w: String, d: String, l: String, p: String) -> String {
    format!("{:<31}| {:^2} | {:^3}| {:^3}| {:^3}| {:^3}", team, mp, w, d, l, p)
}

pub fn format_line_values(team: String, mp: String, w: String, d: String, l: String, p: String) -> String {
    format!("{:<31}| {:^3}| {:^3}| {:^3}| {:^3}| {:^3}", team, mp, w, d, l, p)
}

pub fn tally(match_results: &str) -> String {
    let mut matches_played: HashMap<String, i32> = HashMap::new();
    let mut matches_won: HashMap<String, i32> = HashMap::new();
    let mut matches_drawn: HashMap<String, i32> = HashMap::new();
    let mut matches_lost: HashMap<String, i32> = HashMap::new();
    let mut matches_points: HashMap<String, i32> = HashMap::new();

    let mut output = format_line("Team".to_string(), "MP".to_string(), "W".to_string(), "D".to_string(), "L".to_string(), "P".to_string()).trim_end().to_string();

    if match_results.len() > 0 {
        for line in match_results.split("\n") {
            let split_line = line.split(";").collect::<Vec<&str>>();

            match matches_played.contains_key(split_line[0]) {
                true => {
                    if let Some(matches) = matches_played.get_mut(split_line[0]) {
                        *matches += 1;
                    }
                },
                false => {
                    matches_played.insert(split_line[0].to_string(), 1);
                }
            };
            match matches_played.contains_key(split_line[1]) {
                true => {
                    if let Some(matches) = matches_played.get_mut(split_line[1]) {
                        *matches += 1;
                    }
                },
                false => {
                    matches_played.insert(split_line[1].to_string(), 1);
                }
            };

            match split_line[2] {
                "win" => {
                    match matches_won.contains_key(split_line[0]) {
                        true => {
                            if let Some(matches) = matches_won.get_mut(split_line[0]) {
                                *matches += 1;
                            }
                        },
                        false => {
                            matches_won.insert(split_line[0].to_string(), 1);
                        }
                    };

                    match matches_points.contains_key(split_line[0]) {
                        true => {
                            if let Some(matches) = matches_points.get_mut(split_line[0]) {
                                *matches += 3;
                            }
                        },
                        false => {
                            matches_points.insert(split_line[0].to_string(), 3);
                        }
                    };

                    match matches_points.contains_key(split_line[1]) {
                        true => {
                            if let Some(matches) = matches_points.get_mut(split_line[1]) {
                                *matches += 0;
                            }
                        },
                        false => {
                            matches_points.insert(split_line[1].to_string(), 0);
                        }
                    };

                    match matches_lost.contains_key(split_line[1]) {
                        true => {
                            if let Some(matches) = matches_lost.get_mut(split_line[1]) {
                                *matches += 1;
                            }
                        },
                        false => {
                            matches_lost.insert(split_line[1].to_string(), 1);
                        }
                    };
                },
                "loss" => {
                    match matches_lost.contains_key(split_line[0]) {
                        true => {
                            if let Some(matches) = matches_lost.get_mut(split_line[0]) {
                                *matches += 1;
                            }
                        },
                        false => {
                            matches_lost.insert(split_line[0].to_string(), 1);
                        }
                    };

                    match matches_points.contains_key(split_line[1]) {
                        true => {
                            if let Some(matches) = matches_points.get_mut(split_line[1]) {
                                *matches += 3;
                            }
                        },
                        false => {
                            matches_points.insert(split_line[1].to_string(), 3);
                        }
                    };

                    match matches_points.contains_key(split_line[0]) {
                        true => {
                            if let Some(matches) = matches_points.get_mut(split_line[0]) {
                                *matches += 0;
                            }
                        },
                        false => {
                            matches_points.insert(split_line[0].to_string(), 0);
                        }
                    };

                    match matches_won.contains_key(split_line[1]) {
                        true => {
                            if let Some(matches) = matches_won.get_mut(split_line[1]) {
                                *matches += 1;
                            }
                        },
                        false => {
                            matches_won.insert(split_line[1].to_string(), 1);
                        }
                    };
                },
                "draw" => {
                    match matches_drawn.contains_key(split_line[0]) {
                        true => {
                            if let Some(matches) = matches_drawn.get_mut(split_line[0]) {
                                *matches += 1;
                            }
                        },
                        false => {
                            matches_drawn.insert(split_line[0].to_string(), 1);
                        }
                    };
                    match matches_points.contains_key(split_line[0]) {
                        true => {
                            if let Some(matches) = matches_points.get_mut(split_line[0]) {
                                *matches += 1;
                            }
                        },
                        false => {
                            matches_points.insert(split_line[0].to_string(), 1);
                        }
                    };
                    match matches_drawn.contains_key(split_line[1]) {
                        true => {
                            if let Some(matches) = matches_drawn.get_mut(split_line[1]) {
                                *matches += 1;
                            }
                        },
                        false => {
                            matches_drawn.insert(split_line[1].to_string(), 1);
                        }
                    };
                    match matches_points.contains_key(split_line[1]) {
                        true => {
                            if let Some(matches) = matches_points.get_mut(split_line[1]) {
                                *matches += 1;
                            }
                        },
                        false => {
                            matches_points.insert(split_line[1].to_string(), 1);
                        }
                    };
                }
                _ => ()
            }
        };
    }

    use std::cmp::Ordering;

    if matches_points.len() > 0 {
        let mut items: Vec<(&String, &i32)> = matches_points.iter().collect();
        items.sort_by(|a, b| { //b.1.cmp(a.1));
            match a.1.cmp(b.1).reverse() {
                Ordering::Equal => a.0.cmp(&b.0),
                other => other
            }
        });

        for (team, _) in items {
            let matches_played_val = matches_played.get(team).unwrap_or(&0);
            let matches_won_val = matches_won.get(team).unwrap_or(&0);
            let matches_drawn_val = matches_drawn.get(team).unwrap_or(&0);
            let matches_lost_val = matches_lost.get(team).unwrap_or(&0);
            let matches_points_val = matches_points.get(team).unwrap_or(&0);

            let string = format_line_values(team.to_string(), matches_played_val.to_string(), matches_won_val.to_string(), matches_drawn_val.to_string(), matches_lost_val.to_string(), matches_points_val.to_string()).trim_end().to_string();
            output.push_str("\n");
            output.push_str(&string);
        }

    }

    output
}
