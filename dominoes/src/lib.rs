use petgraph::graph::EdgeIndex;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

pub fn chain(i: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut output: Option<Vec<(u8, u8)>> = Some(Vec::new());
    let mut graphnodes: Vec<NodeIndex> = Vec::new();
    let mut edges: Vec<EdgeIndex> = Vec::new();
    let mut digraph = Graph::<u8, u8>::new();

    let mut pending_nodes: Vec<(u8, u8)> = Vec::new();

    let mut last_domino:(u8, u8) = (0, 0);

    let input = i.to_vec().clone();

    for (index, domino) in input.iter().enumerate() {
        let node_0 = digraph.add_node(domino.0);
        let node_1 = digraph.add_node(domino.1);

        if !graphnodes.is_empty() {
            if last_domino.1 != domino.0  {
                // (_, Y) != (X, _)
                if last_domino.1 != domino.1 {
                    // (_, Y) != (_, P)
                    if (index == 1 || graphnodes.len() == 2) && last_domino.0 == domino.0 {
                        // (X, _) != (X, _)
                        let node_0_index = graphnodes.pop().unwrap();
                        let node_1_index = graphnodes.pop().unwrap();

                        digraph.remove_edge(edges.pop().unwrap());

                        let edge = digraph.add_edge(node_0_index, node_1_index, 10);
                        edges.push(edge);

                        graphnodes.push(node_0_index);
                        graphnodes.push(node_1_index);

                        let gn = graphnodes.clone();
                        let node_last = gn.iter().last().unwrap();

                        let edge_1 = digraph.add_edge(*node_last, node_0, 20);
                        let edge_2 = digraph.add_edge(node_0, node_1, 30);

                        edges.push(edge_1);
                        edges.push(edge_2);

                        graphnodes.push(node_0);
                        graphnodes.push(node_1);

                        last_domino = (domino.0, domino.1);

                    } else if (index == 1 || graphnodes.len() == 2) && last_domino.0 == domino.1 {
                        let node_1_index = graphnodes.pop().unwrap();
                        let node_0_index = graphnodes.pop().unwrap();

                        digraph.remove_edge(edges.pop().unwrap());

                        let edge = digraph.add_edge(node_1_index, node_0_index, 40);
                        edges.push(edge);

                        graphnodes.push(node_1_index);
                        graphnodes.push(node_0_index);

                        let gn = graphnodes.clone();
                        let node_last = gn.iter().last().unwrap();

                        let edge_1 = digraph.add_edge(*node_last, node_1, 50);
                        let edge_2 = digraph.add_edge(node_1, node_0, 60);

                        edges.push(edge_1);
                        edges.push(edge_2);

                        graphnodes.push(node_1);
                        graphnodes.push(node_0);

                        last_domino = (domino.1, domino.0);
                    } else {
                        pending_nodes.push(*domino);
                        output = None;
                    }
                } else {
                    // (_, Y) == (_, Y)
                    let gn = graphnodes.clone();
                    let node_last = gn.iter().last().unwrap();

                    let edge_1 = digraph.add_edge(*node_last, node_1, 90);
                    let edge_2 = digraph.add_edge(node_1, node_0, 100);

                    edges.push(edge_1);
                    edges.push(edge_2);

                    graphnodes.push(node_1);
                    graphnodes.push(node_0);

                    last_domino = (domino.1, domino.0);
                }
            } else {
                // (_, X) == (X, _)
                let gn = graphnodes.clone();
                let node_last = gn.iter().last().unwrap();

                let edge_1 = digraph.add_edge(*node_last, node_0, 110);
                let edge_2 = digraph.add_edge(node_0, node_1, 120);

                edges.push(edge_1);
                edges.push(edge_2);

                graphnodes.push(node_0);
                graphnodes.push(node_1);

                last_domino = *domino;
            }
        } else {
            let edge = digraph.add_edge(node_0, node_1, 130);
            edges.push(edge);

            graphnodes.push(node_0);
            graphnodes.push(node_1);
            last_domino = *domino;
        }
    }

    let mut p = Vec::new();
    if graphnodes.len() > 2 {
        for x in graphnodes.chunks(2) {
            p.push((*digraph.node_weight(x[0]).unwrap(), *digraph.node_weight(x[1]).unwrap()));
        }
    } else if graphnodes.len() == 2 {
        if *digraph.node_weight(graphnodes[0]).unwrap() == *digraph.node_weight(graphnodes[1]).unwrap() {
            p.push((*digraph.node_weight(graphnodes[0]).unwrap(), *digraph.node_weight(graphnodes[1]).unwrap()));
        } else {
            output = None;
        }
    }

    // Try to check if the pending nodes can be inserted.
    if pending_nodes.len() > 0 {
        for (idx, ele) in p.clone().windows(2).enumerate() {
            for (index, pending_node) in pending_nodes.clone().iter().enumerate() {
                if pending_node.0 == ele[0].1 && pending_node.1 == ele[1].0 {
                    p.insert((2 * idx) + 1, pending_node.clone());
                    pending_nodes.remove(index);
                }
            }
        }
    }

    if pending_nodes.len() > 0 {
        pending_nodes.sort();
        match chain(&pending_nodes) {
            Some(value) => {
                for (idx, ele) in p.clone().windows(2).enumerate() {
                    if value[0].0 == ele[0].1 && value[value.len()-1].1 == ele[1].0 {
                        for (index, item) in value.iter().enumerate() {
                            p.insert((2 * idx) + 1 + index, *item);
                        }
                    }
                }

                if value.last().unwrap().1 == p.last().unwrap().1 {
                    for v in value.iter().rev() {
                        p.push((v.1, v.0));
                    }
                }
            },
            None => {

            }
        }
    }

    if p.len() == input.len() {
        Some(p)
    } else {
        None
    }
}
