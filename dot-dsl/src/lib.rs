pub mod graph {
    pub mod graph_items {
        pub mod edge {

            #[derive(PartialEq, Debug)]
            pub struct Edge {
                pub start: String,
                pub end: String,
            }
            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    Edge {
                        start: String::from(start),
                        end: String::from(end),
                    }
                }
            }
        }

        pub mod node {
            #[derive(PartialEq, Debug)]
            pub struct Node {
                pub label: String,
                pub attrs: Vec<(String, String)>,
            }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Node {
                        label: String::from(label),
                        attrs: vec![],
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        self.attrs
                            .push(((*attr).0.to_string(), (*attr).1.to_string()))
                    }
                    self
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;

    #[derive(PartialEq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: Vec<(String, String)>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: vec![],
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            for node in nodes {
                let mut new_node = Node::new(&node.label);
                for attr in &node.attrs {
                    new_node.attrs.push((attr.0.clone(), attr.1.clone()));
                }
                self.nodes.push(new_node)
            }
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            for edge in edges {
                let mut new_edge = Edge::new(&edge.start, &edge.end);

                self.edges.push(new_edge)
            }
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs {
                self.attrs
                    .push(((*attr).0.to_string(), (*attr).1.to_string()))
            }
            self
        }

        pub fn get_node(self, label: &str) -> Self {
            unimplemented!("get_node");
        }
    }
}
