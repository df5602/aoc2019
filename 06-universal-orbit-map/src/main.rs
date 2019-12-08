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

    let minimal_distance = graph.minimal_distance("YOU", "SAN");
    println!("Minimal distance: {}", minimal_distance - 2);
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
    parent: String,
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
            let node = graph.nodes.entry(orbit.object.clone()).or_insert(Node {
                object: orbit.object.clone(),
                parent: String::new(),
                children: Vec::new(),
            });
            assert!(node.parent.is_empty());
            node.parent = orbit.center.clone();

            let parent = graph.nodes.entry(orbit.center.clone()).or_insert(Node {
                object: orbit.center.clone(),
                parent: String::new(),
                children: Vec::new(),
            });
            parent.children.push(orbit.object.clone());
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

    fn minimal_distance(&self, from: &str, to: &str) -> usize {
        let mut visited: HashMap<String, usize> = HashMap::new();

        // Start at from node and go up the parents
        let mut distance = 0;
        let mut current = self.nodes.get(from).unwrap();
        visited.insert(current.object.clone(), distance);

        loop {
            if current.parent.is_empty() {
                break;
            }
            current = self.nodes.get(&current.parent).unwrap();

            distance += 1;
            visited.insert(current.object.clone(), distance);
        }

        // Start at to node and go up the parents
        distance = 0;
        current = self.nodes.get(to).unwrap();

        loop {
            if visited.contains_key(&current.object) {
                return distance + visited.get(&current.object).unwrap();
            }

            distance += 1;
            current = self.nodes.get(&current.parent).unwrap();
            if current.parent.is_empty() {
                break;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let orbits: Vec<Orbit> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();
        let graph = Graph::construct_graph(&orbits);
        let number_of_orbits = graph.count_orbits();
        assert_eq!(621125, number_of_orbits);
    }

    #[test]
    fn part_2() {
        let orbits: Vec<Orbit> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();
        let graph = Graph::construct_graph(&orbits);
        let minimal_distance = graph.minimal_distance("YOU", "SAN");
        assert_eq!(550, minimal_distance - 2);
    }
}
