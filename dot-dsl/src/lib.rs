
pub mod graph {
    use std::fmt;
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct Graph {
        pub edges: Vec<graph_items::edge::Edge>,
        pub nodes: Vec<graph_items::node::Node>,
        pub attrs: HashMap<String,String>
    }

    pub mod graph_items {
        pub mod edge {
            use std::fmt;
            use std::collections::HashMap;

            #[derive(Debug, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String,String>
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Edge {
                    for item in attrs {
                        self.attrs.insert(item.0.to_string(), item.1.to_string());
                    }
                    self.clone()
                }

            }

            impl fmt::Display for Edge {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{:?}->{:?}", self.from, self.to)
                }
            }

            impl PartialEq for Edge {
                fn eq(&self, other: &Self) -> bool {
                    (self.from == other.from ) && (self.to == other.to)
                }
            }

        }

        pub mod node {
            use std::collections::HashMap;
            use std::fmt;

            #[derive(Debug, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String,String>
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new()
                    }
                }
                pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> Node {
                    for item in attrs {
                        self.attrs.insert(item.0.to_string(), item.1.to_string());
                    }
                    self.clone()
                }
                pub fn get_attr(&mut self, key: &str) -> Option<&str> {
                    match self.attrs.get(key) {
                        Some(x) => Some(&x),
                        None => None
                    }

                }
            }

            impl fmt::Display for Node {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{:?}", self.name)
                }
            }

            impl PartialEq for Node {
                fn eq(&self, other: &Self) -> bool {
                    self.name == other.name
                }
            }

        }
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new()
            }
        } 

        pub fn with_nodes(&mut self, nodes: & Vec<graph_items::node::Node>) -> Graph {
            self.nodes.clone_from(nodes);
            self.clone()
        }

        pub fn with_edges(&mut self, edges: & Vec<graph_items::edge::Edge>) -> Graph {
            self.edges.clone_from(edges);
            self.clone()
        }

        pub fn with_attrs(&mut self, attrs: &[(& str, & str)]) -> Graph {
            for item in attrs {
                self.attrs.insert(item.0.to_string(), item.1.to_string());
            }
            self.clone()
        }

        pub fn get_node(&mut self, name: &str) -> Option<graph_items::node::Node> {
            let mut nodes: Vec<graph_items::node::Node> = self.nodes.clone().into_iter()
                .filter(|x| x.name == name)
                .collect();

            nodes.pop()
        }
    }
}
