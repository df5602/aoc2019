use std::collections::HashMap;
use std::env;

use adhoc_derive::FromStr;
use aoc_util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let orbits: Vec<Orbit> = match FileReader::new().split_lines().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let graph = Graph::construct_graph(&orbits);
    let number_of_orbits = graph.count_orbits();
    println!("Number of orbits: {}", number_of_orbits);
}

#[derive(Debug, FromStr)]
#[adhoc(regex = r"^(?P<center>.+)\)(?P<object>.+)$")]
struct Orbit {
    object: String,
    center: String,
}

#[derive(Debug)]
struct Node {
    object: String,
    children: Vec<String>,
}

struct Graph {
    root: String,
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn construct_graph(orbits: &[Orbit]) -> Self {
        let mut graph = Self {
            root: String::from("COM"),
            nodes: HashMap::new(),
        };
        for orbit in orbits {
            graph.nodes.entry(orbit.object.clone()).or_insert(Node {
                object: orbit.object.clone(),
                children: Vec::new(),
            });
            let parent = graph.nodes.entry(orbit.center.clone()).or_insert(Node {
                object: orbit.center.clone(),
                children: Vec::new(),
            });
            (*parent).children.push(orbit.object.clone());
        }
        graph
    }

    fn count_orbits(&self) -> usize {
        let root = self.nodes.get(&self.root).unwrap();
        self.count_depth(&root, 0)
    }

    fn count_depth(&self, node: &Node, depth: usize) -> usize {
        let mut sum = depth;
        for child in &node.children {
            let node = self.nodes.get(child).unwrap();
            sum += self.count_depth(&node, depth + 1);
        }
        sum
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_depth() {

     }
}*/
