mod graph_test_lib {
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
            let mut visited = HashMap::new();
            let mut queue = VecDeque::new();
            queue.push_back(start);
            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();
                if !visited.contains_key(&node) {
                    visited.insert(node, Vec::new());
                    for edge in &self.nodes.iter().find(|n| n.id == node).unwrap().edges {
                        queue.push_back(*edge);
                        visited.get_mut(&node).unwrap().push(*edge);
                    }
                }
            }
            visited
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
                let mut node = Node::new(id, data);
                node.edges = edges;
                nodes.push(Node::new(id, data));
            }
            graph.add_nodes(nodes);
            graph
        }
    }

    impl<T> Node<T> where T : Debug + Display + ToString {
        pub fn new(id: i32, data: T) -> Node<T> {
            Node {
                id: id,
                data: data,
                edges: Vec::new(),
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
        let mut node1 = Node::new(1, "Node 1");
        let mut node2 = Node::new(2, "Node 2");
        let mut node3 = Node::new(3, "Node 3");

        node1.add_edge(2);
        node1.add_edge(3);
        node2.add_edge(1);
        node3.add_edge(1);

        let mut graph = Graph::new();
        graph.add_nodes(vec![node1, node2, node3]);

        assert_eq!(graph.nodes.len(), 3);
    }

    #[test]
    #[should_panic]
    fn duplicate_node_id() {
        let node1 = Node::new(1, "Node 1");
        let node2 = Node::new(1, "Node 2");

        let mut graph = Graph::new();
        graph.add_nodes(vec![node1, node2]);
    }

    #[test]
    #[should_panic]
    fn non_existing_edge() {
        let mut node1 = Node::new(1, "Node 1");
        let node2 = Node::new(2, "Node 2");

        node1.add_edge(2);
        node1.add_edge(3);

        let mut graph = Graph::new();
        graph.add_nodes(vec![node1, node2]);
    }

    #[test]
    fn simple_bfs() {
        let mut node1 = Node::new(1, "Node 1");
        let mut node2 = Node::new(2, "Node 2");
        let mut node3 = Node::new(3, "Node 3");
        let node4 = Node::new(4, "Node 4");
        let node5 = Node::new(5, "Node 5");
        let node6 = Node::new(6, "Node 6");
        let node7 = Node::new(7, "Node 7");

        node1.add_edge(2);
        node1.add_edge(3);
        node2.add_edge(4);
        node2.add_edge(5);
        node3.add_edge(6);
        node3.add_edge(7);

        let mut graph = Graph::new();
        graph.add_nodes(vec![node1, node2, node3, node4, node5, node6, node7]);

        let result = graph.breadth_first_search(1);
        assert_eq!(result.len(), 7);
        assert_eq!(result.get(&1).unwrap().len(), 2);
        assert_eq!(result.get(&2).unwrap().len(), 2);
        assert_eq!(result.get(&3).unwrap().len(), 2);
        assert_eq!(result.get(&4).unwrap().len(), 0);
        assert_eq!(result.get(&5).unwrap().len(), 0);
        assert_eq!(result.get(&6).unwrap().len(), 0);
        assert_eq!(result.get(&7).unwrap().len(), 0);
    }

    #[test]
    fn serialization() {
        let mut node1 = Node::new(1, "Node");
        let mut node2 = Node::new(2, "Node");
        let mut node3 = Node::new(3, "Node");
        let node4 = Node::new(4, "Node");
        let node5 = Node::new(5, "Node");
        let node6 = Node::new(6, "Node");
        let node7 = Node::new(7, "Node");

        node1.add_edge(2);
        node1.add_edge(3);
        node2.add_edge(4);
        node2.add_edge(5);
        node3.add_edge(6);
        node3.add_edge(7);

        let mut graph = Graph::new();
        graph.add_nodes(vec![node1, node2, node3, node4, node5, node6, node7]);

        let result = graph.serialization();
        assert_eq!(result, "1 Node 2 3 \n2 Node 4 5 \n3 Node 6 7 \n4 Node \n5 Node \n6 Node \n7 Node \n");
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
    }
}
