use std::{fs::File, io::Read};

use graph_test_app::graph_test_lib::*;

fn main() {
    let mut file = File::open("src/test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let graph: Graph<&str> = Graph::<&str>::deserialization(&contents);
    graph_traversal(graph);
}

//bypass its vertices and for each of them output its identifier, identifiers of adjacent vertices and its values to the console
fn graph_traversal(graph: Graph<&str>) {
    let mut visited = vec![false; graph.nodes.len()];
    let mut stack = Vec::new();
    stack.push(0);
    while stack.len() > 0 {
        let node = stack.pop().unwrap();
        if !visited[node] {
            visited[node] = true;
            println!("{} {} {:?}", graph.nodes[node].id, graph.nodes[node].data, graph.nodes[node].edges);
            for edge in &graph.nodes[node].edges {
                stack.push(*edge as usize);
            }
        }
    }
}