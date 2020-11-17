pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(PartialEq, Debug)]
            pub struct Edge {
                pub start: String,
                pub end: String,
                pub attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    Edge {
                        start: String::from(start),
                        end: String::from(end),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        self.attrs
                            .insert((*attr).0.to_string(), (*attr).1.to_string());
                    }
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Debug)]
            pub struct Node {
                pub label: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Node {
                        label: String::from(label),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        self.attrs
                            .insert((*attr).0.to_string(), (*attr).1.to_string());
                    }
                    self
                }

                pub fn get_attr(self, attr: &str) -> Option<String> {
                    match self.attrs.get(&attr.to_string()) {
                        None => None,
                        Some(s) => Some(s.to_string()),
                    }
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(PartialEq, Debug, Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            for node in nodes {
                let mut new_node = Node::new(&node.label);
                for attr in &node.attrs {
                    new_node.attrs.insert(attr.0.clone(), attr.1.clone());
                }
                self.nodes.push(new_node)
            }
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            for edge in edges {
                let mut new_edge = Edge::new(&edge.start, &edge.end);
                for attr in &edge.attrs {
                    new_edge.attrs.insert(attr.0.clone(), attr.1.clone());
                }
                self.edges.push(new_edge)
            }
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs {
                self.attrs
                    .insert((*attr).0.to_string(), (*attr).1.to_string());
            }
            self
        }

        pub fn get_node(self, label: &str) -> Option<Node> {
            let matching_node = self.nodes.iter().find(|node| node.label == label);
            match matching_node {
                None => None,
                Some(node) => {
                    let mut new_node = Node::new(&node.label);
                    for attr in &node.attrs {
                        new_node.attrs.insert(attr.0.clone(), attr.1.clone());
                    }
                    Some(new_node)
                }
            }
        }
    }
}
