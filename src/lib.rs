pub mod graph_test_lib {
    use std::collections::{VecDeque, HashMap};
    use std::fmt::Debug;
    use std::fmt::Display;
    pub struct Graph<T> where T : Debug + Display + ToString {
        pub nodes: Vec<Node<T>>,
    }
    pub struct Node<T> where T : Debug + Display + ToString {
        pub id: i32,
        pub data: T,
        pub edges: Vec<i32>,
    }

    impl<T : Debug + Display + ToString> Graph<T> where T : Debug + Display + ToString {
        pub fn new() -> Graph<T> {
            Graph { nodes: Vec::new() }
        }
        pub fn add_nodes(&mut self, nodes: Vec<Node<T>>) {
            for node in nodes {
                self.nodes.push(node);
            }
            // check that all nodes edges to existing nodes
            for node in &self.nodes {
                for edge in &node.edges {
                    if !self.nodes.iter().any(|n| n.id == *edge) {
                        panic!("Node {} has edge to non-existing node {}", node.id, edge);
                    }
                }
            }

            // check that all nodes have unique id
            let mut ids = Vec::new();
            for node in &self.nodes {
                ids.push(node.id);
            }
            ids.sort();
            ids.dedup();
            if ids.len() != self.nodes.len() {
                panic!("Graph has nodes with non-unique id");
            }
        }
        pub fn breadth_first_search(&self, start: i32) -> HashMap<i32, Vec<i32>> {
            let mut visited = vec![false; self.nodes.len()];
            let mut queue = VecDeque::new();
            let mut result = HashMap::new();
            queue.push_back(start);
            while queue.len() > 0 {
                let node = queue.pop_front().unwrap() - 1;
                if !visited[node as usize] {
                    visited[node as usize] = true;
                    result.insert(node, self.nodes[node as usize].edges.clone());
                    for edge in &self.nodes[node as usize].edges {
                        queue.push_back(*edge);
                    }
                }
            }
            result
        }
        // Trivial Graph Format
        pub fn serialization(&self) -> String {
            let mut result = String::new();
            for node in &self.nodes {
                result.push_str(&format!("{} ", node.id));
                result.push_str(&format!("{} ", node.data.to_string().as_str()));
                if node.edges.len() > 0 { 
                    for edge in &node.edges {
                        result.push_str(&format!("{} ", edge));
                    }
                }
                // make new line
                result.push('\n');
            }
            result
        }

        pub fn deserialization(serialized: &str) -> Graph<&str> {
            let mut graph = Graph::new();
            let mut nodes = Vec::new();
            for line in serialized.lines() {
                let mut parts = line.split_whitespace();
                let id = parts.next().unwrap().parse::<i32>().unwrap();
                let data = parts.next().unwrap();
                let mut edges = Vec::new();
                for part in parts {
                    edges.push(part.parse::<i32>().unwrap());
                }
                let node = Node::new(id, data, edges);
                nodes.push(node);
            }
            graph.add_nodes(nodes);
            graph
        }
    }

    impl<T> Node<T> where T : Debug + Display + ToString {
        pub fn new(id: i32, data: T, edges: Vec<i32>) -> Node<T> {
            Node {
                id: id,
                data: data,
                edges: edges,
            }
        }
        pub fn add_edge(&mut self, edge: i32) {
            self.edges.push(edge);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::graph_test_lib::*;

    #[test]
    fn simple_creating() {
        let node = Node::new(1, "test", vec![2, 3]);

        assert_eq!(node.id, 1);
        assert_eq!(node.data, "test");
        assert_eq!(node.edges, vec![2, 3]);
    }

    #[test]
    #[should_panic]
    fn duplicate_node_id() {
        let mut graph = Graph::new();
        let mut nodes = Vec::new();
        nodes.push(Node::new(1, "a", vec![2, 3]));
        nodes.push(Node::new(1, "b", vec![3]));
        nodes.push(Node::new(3, "c", vec![]));
        graph.add_nodes(nodes);
    }

    #[test]
    #[should_panic]
    fn non_existing_edge() {
        let mut graph = Graph::new();
        let mut nodes = Vec::new();
        nodes.push(Node::new(1, "a", vec![2, 3]));
        nodes.push(Node::new(2, "b", vec![3]));
        nodes.push(Node::new(3, "c", vec![4]));
        graph.add_nodes(nodes);
    }

    #[test]
    fn simple_bfs() {
        let mut graph = Graph::new();
        let mut nodes = Vec::new();
        nodes.push(Node::new(1, "a", vec![2, 3]));
        nodes.push(Node::new(2, "b", vec![3]));
        nodes.push(Node::new(3, "c", vec![]));
        graph.add_nodes(nodes);
        let visited = graph.breadth_first_search(1);
        assert_eq!(visited.len(), 3);
        assert_eq!(visited.get(&0).unwrap(), &vec![2, 3]);
        assert_eq!(visited.get(&1).unwrap(), &vec![3]);
        assert_eq!(visited.get(&2).unwrap(), &vec![]);
    }

    #[test]
    fn serialization() {
        let mut graph = Graph::new();
        let mut nodes = Vec::new();
        nodes.push(Node::new(1, "a", vec![2, 3]));
        nodes.push(Node::new(2, "b", vec![3]));
        nodes.push(Node::new(3, "c", vec![]));
        graph.add_nodes(nodes);
        let serialized = graph.serialization();
        assert_eq!(serialized, "1 a 2 3 \n2 b 3 \n3 c \n");
    }

    #[test]
    fn deserialization() {
        let serialized = "1 Node 2 3
                                2 Node 4 5
                                3 Node 6 7
                                4 Node 
                                5 Node 
                                6 Node 
                                7 Node ";
        let graph = Graph::<&str>::deserialization(serialized);
        assert_eq!(graph.nodes.len(), 7);
        assert_eq!(graph.nodes.get(1).unwrap().edges.len(), 2);
        assert_eq!(graph.nodes.get(2).unwrap().edges.len(), 2);
        assert_eq!(graph.nodes.get(3).unwrap().edges.len(), 0);
    }
}
