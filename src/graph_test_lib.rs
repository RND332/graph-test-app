mod graph_test_app {
    use std::collections::{VecDeque, HashMap};

    pub struct Graph<T> {
        pub nodes: Vec<Node<T>>,
    }
    pub struct Node<T> {
        pub id: i32,
        pub data: T,
        pub edges: Vec<i32>,
    }

    impl<T> Graph<T> {
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
    }

    impl<T> Node<T> {
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
    use crate::graph_test_app;

    #[test]
    fn simple_creating() {
        let mut node1 = graph_test_app::Node::new(1, "Node 1");
        let mut node2 = graph_test_app::Node::new(2, "Node 2");
        let mut node3 = graph_test_app::Node::new(3, "Node 3");

        node1.add_edge(2);
        node1.add_edge(3);
        node2.add_edge(1);
        node3.add_edge(1);

        let mut graph = graph_test_app::Graph::new();
        graph.add_nodes(vec![node1, node2, node3]);

        assert_eq!(graph.nodes.len(), 3);
    }

    #[test]
    #[should_panic]
    fn duplicate_node_id() {
        let node1 = graph_test_app::Node::new(1, "Node 1");
        let node2 = graph_test_app::Node::new(1, "Node 2");

        let mut graph = graph_test_app::Graph::new();
        graph.add_nodes(vec![node1, node2]);
    }

    #[test]
    #[should_panic]
    fn non_existing_edge() {
        let mut node1 = graph_test_app::Node::new(1, "Node 1");
        let node2 = graph_test_app::Node::new(2, "Node 2");

        node1.add_edge(2);
        node1.add_edge(3);

        let mut graph = graph_test_app::Graph::new();
        graph.add_nodes(vec![node1, node2]);
    }

    #[test]
    fn simple_bfs() {
        let mut node1 = graph_test_app::Node::new(1, "Node 1");
        let mut node2 = graph_test_app::Node::new(2, "Node 2");
        let mut node3 = graph_test_app::Node::new(3, "Node 3");
        let node4 = graph_test_app::Node::new(4, "Node 4");
        let node5 = graph_test_app::Node::new(5, "Node 5");
        let node6 = graph_test_app::Node::new(6, "Node 6");
        let node7 = graph_test_app::Node::new(7, "Node 7");

        node1.add_edge(2);
        node1.add_edge(3);
        node2.add_edge(4);
        node2.add_edge(5);
        node3.add_edge(6);
        node3.add_edge(7);

        let mut graph = graph_test_app::Graph::new();
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
}
