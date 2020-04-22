extern crate petgraph;
extern crate onig;

use petgraph::graph::NodeIndex;
use onig::*;
use std::collections::HashMap;
use petgraph::Graph;
use petgraph::dot::{Dot, Config};

pub fn convert_horizontal_patterns_to_graph(graph: &mut Graph::<(usize, usize), u32, petgraph::Undirected>,
    lines: &[&str],
    delimiter: char,
    continous: char) -> (Graph::<(usize, usize), u32, petgraph::Undirected>,  HashMap<(usize, usize), NodeIndex>) {

    let deps = graph;
    let mut node_index: HashMap<(usize, usize), NodeIndex> = HashMap::new();

    for (index, line) in lines.iter().enumerate() {
        let re = Regex::new(&"\\-1[-2]*\\-1"
            .replace("-1", &delimiter.to_string())
            .replace("-2", &continous.to_string()))
            .unwrap();
        let re_with_lookfwd = Regex::new(&"(?=(\\-1[-1-2]*\\-1))"
            .replace("-1", &delimiter.to_string())
            .replace("-2", &continous.to_string()))
            .unwrap();

        let input_string_line = line.clone();

        for ch in re_with_lookfwd.find_iter(&input_string_line) {
            let mut end_marker = 0;
            for (tempindex, iter) in input_string_line[ch.0 ..= input_string_line.len() - 1].chars().enumerate() {
                // println!("Find the onwards pattern : {:?} {:?}", ch, end_marker);
                if iter != continous && iter != delimiter {
                    end_marker = ch.0 + tempindex - 1;
                    break;
                } else {
                    end_marker = ch.0 + tempindex;
                }

                if  iter == delimiter && tempindex > 0 {
                    end_marker = ch.0 + tempindex;
                    // println!("++++++++++++++ {:?} -> {:?}", (index, ch.0), (index, end_marker));
                    let node_left;
                    let node_right;

                    match node_index.contains_key(&(index, ch.0)) {
                        true => {
                            node_left = *node_index.get(&(index, ch.0)).unwrap();
                        }, false => {
                            node_left = deps.add_node((index, ch.0));
                            node_index.insert((index, ch.0), node_left);
                        }
                    }

                    match node_index.contains_key(&(index, end_marker)) {
                        true => {
                            node_right = *node_index.get(&(index, end_marker)).unwrap();
                        }, false => {
                            node_right = deps.add_node((index, end_marker));
                            node_index.insert((index, end_marker), node_right);
                        }
                    }

                    let edge_weight = end_marker as u32 - ch.0 as u32;

                    match deps.contains_edge(node_left, node_right) {
                        true => {
                            // ignore adding the edge.
                        },
                        false => {
                            deps.add_edge(node_left, node_right, edge_weight);
                        }
                    }

                    println!("Horizontal {:?} -> {:?}, {:?}, {:?} {}",
                        end_marker, deps.node_weight(node_left),
                        deps.node_weight(node_right),
                        ch,
                        &input_string_line[ch.0..=end_marker]);
                }
            }
            let node_left;
            let node_right;

            match node_index.contains_key(&(index, ch.0)) {
                true => {
                    node_left = *node_index.get(&(index, ch.0)).unwrap();
                }, false => {
                    node_left = deps.add_node((index, ch.0));
                    node_index.insert((index, ch.0), node_left);
                }
            }

            match node_index.contains_key(&(index, end_marker)) {
                true => {
                    node_right = *node_index.get(&(index, end_marker)).unwrap();
                }, false => {
                    node_right = deps.add_node((index, end_marker));
                    node_index.insert((index, end_marker), node_right);
                }
            }

            let edge_weight = end_marker as u32 - ch.0 as u32;

            match deps.contains_edge(node_left, node_right) {
                true => {
                    // ignore adding the edge.
                },
                false => {
                    deps.add_edge(node_left, node_right, edge_weight);
                }
            }

            println!("Horizontal {:?} -> {:?}, {:?}, {:?} {}",
                end_marker, deps.node_weight(node_left),
                deps.node_weight(node_right),
                ch,
                &input_string_line[ch.0..=end_marker]);
        }

        let mut marker = 0;

        while marker < line.len() {
            // println!("Line -> {:?}", (line, marker));
            match re.find(&line[marker..=line.len()-1]) {
                Some(x) => {
                    // println!("String found {:?}", x);
                    println!("String found {:?} -> {:?}", (marker + x.0, marker + x.1 - 1), &line[marker + x.0..= marker + x.1-1]);

                    let node_left;
                    let node_right;

                    match node_index.contains_key(&(index, marker + x.0)) {
                        true => {
                            node_left = *node_index.get(&(index, marker + x.0)).unwrap();
                        }, false => {
                            node_left = deps.add_node((index, marker + x.0));
                            node_index.insert((index, marker + x.0), node_left);
                        }
                    }

                    match node_index.contains_key(&(index, marker+ x.1 -1)) {
                        true => {
                            node_right = *node_index.get(&(index, marker+ x.1 -1)).unwrap();
                        }, false => {
                            node_right = deps.add_node((index, marker+ x.1 -1));
                            node_index.insert((index, marker+ x.1 -1), node_right);
                        }
                    }

                    let edge_weight = (marker + x.1 - 1) as u32 - (marker + x.0) as u32;

                    match deps.contains_edge(node_left, node_right) {
                        true => {
                            // ignore adding the edge.
                        },
                        false => {
                            deps.add_edge(node_left, node_right, edge_weight);
                        }
                    }

                    marker += x.1 - 1;
                },
                None => {
                    break;
                }
            }
        }
    }
    (deps.clone(), node_index.clone())
}

pub fn convert_vertical_patterns_to_graph(graph: &mut Graph::<(usize, usize), u32, petgraph::Undirected>,
    lines: &[&str],
    delimiter: char,
    continous: char,
    node_index: &mut HashMap<(usize, usize), NodeIndex>) -> Graph::<(usize, usize), u32, petgraph::Undirected> {

    let deps = graph;

    let colsize = match lines.len() {
        0 => 0,
        _ => lines[0].len()
    };

    for index in 0..colsize {
        let mut line = String::new();

        for rowindex in 0..lines.len() {
            line.push(lines[rowindex].chars().nth(index).unwrap());
        }

        let re = Regex::new(&"\\-1[-2]*\\-1".replace("-1", &delimiter.to_string()).replace("-2", &continous.to_string())).unwrap();
        let re_with_lookfwd = Regex::new(&"(?=(\\-1[-1-2]*\\-1))".replace("-1", &delimiter.to_string()).replace("-2", &continous.to_string())).unwrap();

        let input_string_line = line.clone();

        for ch in re_with_lookfwd.find_iter(&input_string_line) {
            let mut end_marker = 0;
            for (rowindex, iter) in input_string_line[ch.0 ..= input_string_line.len() - 1].chars().enumerate() {
                if iter != continous && iter != delimiter {
                    end_marker = ch.0 + rowindex - 1;
                    break;
                }

                if  iter == delimiter && rowindex > 0{
                    end_marker = ch.0 + rowindex;

                    let node_left;
                    let node_right;

                    match node_index.contains_key(&(ch.0, index)) {
                        true => {
                            node_left = *node_index.get(&(ch.0, index)).unwrap();
                        }, false => {
                            node_left = deps.add_node((ch.0, index));
                            node_index.insert((ch.0, index), node_left);
                        }
                    }

                    match node_index.contains_key(&(end_marker, index)) {
                        true => {
                            node_right = *node_index.get(&(end_marker, index)).unwrap();
                        }, false => {
                            node_right = deps.add_node((end_marker, index));
                            node_index.insert((end_marker, index), node_right);
                        }
                    }

                    let edge_weight = end_marker as u32 - ch.0 as u32;

                    match deps.contains_edge(node_left, node_right) {
                        true => {
                            // ignore adding the edge.
                        },
                        false => {
                            deps.extend_with_edges(&[(node_left, node_right, edge_weight)]);
                        }
                    }
                }
            }

            let node_left;
            let node_right;

            match node_index.contains_key(&(ch.0, index)) {
                true => {
                    node_left = *node_index.get(&(ch.0, index)).unwrap();
                }, false => {
                    node_left = deps.add_node((ch.0, index));
                    node_index.insert((ch.0, index), node_left);
                }
            }

            match node_index.contains_key(&(end_marker, index)) {
                true => {
                    node_right = *node_index.get(&(end_marker, index)).unwrap();
                }, false => {
                    node_right = deps.add_node((end_marker, index));
                    node_index.insert((end_marker, index), node_right);
                }
            }

            let edge_weight = end_marker as u32 - ch.0 as u32;

            match deps.contains_edge(node_left, node_right) {
                true => {
                    // ignore adding the edge.
                },
                false => {
                    deps.extend_with_edges(&[(node_left, node_right, edge_weight)]);
                }
            }
        }

        let mut marker = 0;
        let line_cloned = &line.clone();

        while marker < line_cloned.len() {
            // println!("Line -> {:?}", (line_cloned, marker));
            match re.find(&line_cloned[marker..=line_cloned.len()-1]) {
                Some(x) => {
                    let node_left;
                    let node_right;

                    match node_index.contains_key(&(marker + x.0, index)) {
                        true => {
                            node_left = *node_index.get(&(marker + x.0, index)).unwrap();
                        }, false => {
                            node_left = deps.add_node((marker + x.0, index));
                            node_index.insert((marker + x.0, index), node_left);
                        }
                    }

                    match node_index.contains_key(&(marker+ x.1 -1, index)) {
                        true => {
                            node_right = *node_index.get(&(marker+ x.1 -1, index)).unwrap();
                        }, false => {
                            node_right = deps.add_node((marker+ x.1 -1, index));
                            node_index.insert((marker+ x.1 -1, index), node_right);
                        }
                    }

                    let edge_weight = (marker + x.1 - 1) as u32 - (marker + x.0) as u32;

                    match deps.contains_edge(node_left, node_right) {
                        true => {
                            // ignore adding the edge.
                        },
                        false => {
                            deps.extend_with_edges(&[(node_left, node_right, edge_weight)]);
                        }
                    }

                    marker += x.1-1;
                },
                None => {
                    break;
                }
            }
        }
    }
    deps.clone()
}

extern crate itertools;
use itertools::Itertools;

pub fn count(lines: &[&str]) -> u32 {
    let mut deps = Graph::<(usize, usize), u32, petgraph::Undirected>::new_undirected();

    let (mut graph, mut node_index) = convert_horizontal_patterns_to_graph(&mut deps, lines, '+', '-');
    let graph_w_h = convert_vertical_patterns_to_graph(&mut graph, lines, '+', '|', &mut node_index);
    let indexes = graph_w_h.node_indices().collect::<Vec<_>>();
    let mut output_vector = Vec::new();

    for start in indexes {
        let root_node_weight = graph_w_h.node_weight(start).unwrap();
        let neighbors = graph_w_h.neighbors_undirected(start).collect::<Vec<_>>();

        if neighbors.len() >= 2 {
            for neighbor in neighbors.iter().combinations(2) {
                let node_start = neighbor[0];
                let node_end = neighbor[1];

                let node_start_neighbor_node_weight = graph_w_h.node_weight(*node_start).unwrap();
                let node_start_neighbor_edge = graph_w_h.find_edge_undirected(start, *node_start).unwrap();
                let node_start_neighbor_edge_weight = graph_w_h.edge_weight(node_start_neighbor_edge.0).unwrap();

                let node_end_neighbor_node_weight = graph_w_h.node_weight(*node_end).unwrap();
                let node_end_neighbor_edge = graph_w_h.find_edge_undirected(start, *node_end).unwrap();
                let node_end_neighbor_edge_weight = graph_w_h.edge_weight(node_end_neighbor_edge.0).unwrap();

                let target_weight_start = node_end_neighbor_edge_weight;
                let target_weight_end = node_start_neighbor_edge_weight;

                pub fn pursuit (graph_w_h: &Graph::<(usize, usize), u32, petgraph::Undirected>,
                    node: NodeIndex,
                    path: Vec<(usize, usize)>,
                    sum: u32,
                    goal: u32,
                    node_end: NodeIndex,
                    node_start: NodeIndex,
                    output_vector: &mut std::vec::Vec<std::vec::Vec<(usize, usize)>>) -> Option<(Vec<(usize, usize)>, u32)> {

                    let mut res: Option<(Vec<(usize, usize)>, u32)> = None;

                    if sum == goal {
                        let node_neighbor_edge_end = graph_w_h.find_edge_undirected(node, node_end);
                        let node_neighbor_edge_orig_start = graph_w_h.find_edge_undirected(node, node_start);
                        if node_neighbor_edge_end != None && node_neighbor_edge_orig_start != None {
                            //println!("End pursuit : path = {:?} sum = {:?} goal = {:?} ", path, sum, goal);
                            let mut temp_entry = path.clone();

                            temp_entry.sort();
                            temp_entry.dedup();

                            if temp_entry.len() == 4 {
                                output_vector.push(temp_entry);
                            }

                            res = Some((path, sum));
                        }
                    } else if sum > goal || path.contains(graph_w_h.node_weight(node).unwrap()) && sum != 0 {
                        res = None;
                    }
                    else {
                        let neighbors_node = graph_w_h.neighbors_undirected(node).collect::<Vec<_>>();
                        if neighbors_node.len() >= 2 {
                            for neighbour_node in neighbors_node {
                                let node_neighbor_node_weight = graph_w_h.node_weight(neighbour_node).unwrap();
                                let node_neighbor_node_weight_end = graph_w_h.node_weight(node_end).unwrap();
                                let node_neighbor_edge_start = graph_w_h.find_edge_undirected(node, neighbour_node).unwrap();
                                let node_neighbor_edge_weight = graph_w_h.edge_weight(node_neighbor_edge_start.0).unwrap();

                                let node_neighbor_edge_end = graph_w_h.find_edge_undirected(neighbour_node, node_end);
                                let node_neighbor_edge_orig_start = graph_w_h.find_edge_undirected(neighbour_node, node_start);

                                if node_neighbor_edge_end != None { // && node_neighbor_edge_orig_start != None {
                                    if !path.contains(node_neighbor_node_weight) {

                                        let mut path = path.clone();
                                        path.push(*node_neighbor_node_weight);

                                        res = pursuit(&graph_w_h, neighbour_node, path, sum + node_neighbor_edge_weight, goal, node_end, node_start, output_vector);
                                    }
                                } else {
                                    res = None
                                }
                            }
                        }
                    }

                    res
                };
                let mut path = Vec::new();
                path.push(*root_node_weight);
                path.push(*node_start_neighbor_node_weight);
                path.push(*node_end_neighbor_node_weight);

                pursuit(&graph_w_h, *node_start, path, 0, *target_weight_start, *node_end, *node_start, &mut output_vector);

                let mut path = Vec::new();
                path.push(*root_node_weight);
                path.push(*node_start_neighbor_node_weight);
                path.push(*node_end_neighbor_node_weight);

                pursuit(&graph_w_h, *node_end, path, 0, *target_weight_end, *node_end, *node_start, &mut output_vector);
            }
        }
    }

    println!("Captured rectangles ... ");
    for out in output_vector.clone() {
        //println!("--> {:?}", out);
    }

    output_vector.sort();
    output_vector.dedup();

    let mut count = 0;

    for out in output_vector {
        let distance_left: f64 = ((out[1].0 as f64 - out[0].0 as f64).powf(2.0) + (out[1].1 as f64 - out[0].1 as f64).powf(2.0)).sqrt();
        let distance_right: f64 = ((out[3].0 as f64 - out[2].0 as f64).powf(2.0) + (out[3].1 as f64 - out[2].1 as f64).powf(2.0)).sqrt();

        let distance_top: f64 = ((out[3].0 as f64 - out[0].0 as f64).powf(2.0) + (out[3].1 as f64- out[2].1 as f64).powf(2.0)).sqrt();
        let distance_down: f64 = ((out[2].0 as f64 - out[1].0 as f64).powf(2.0) + (out[2].1 as f64- out[1].1 as f64).powf(2.0)).sqrt();

        if (distance_left == distance_right) && (distance_top == distance_down) {
            count += 1;
        } else {
            println!("Dropped rectangles {:?} -> {:?} {:?}", out, (distance_left, distance_right), (distance_top, distance_down));
        }
    }

    println!("{:?}", Dot::with_config(&graph_w_h, &[]));

    count
}
