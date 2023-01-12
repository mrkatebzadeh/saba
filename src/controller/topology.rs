use std::collections::HashMap;
use std::collections::VecDeque;
use crate::switch::{NetworkNode, Switch, Server};
use log::{debug, error, info, warn};

#[derive(Debug)]
pub struct Topology {
    nodes: HashMap<String,  Box<dyn NetworkNode> >,
    adjacency: HashMap<String, Vec<String>>,
}

impl Topology {
    fn dfs(&self, start: &str, end: &str) -> Option<Vec<String>> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        let mut path = vec![];

        queue.push_back(start);
        visited.insert(start, true);

        while !queue.is_empty() {
            let &current = self.nodes[queue.pop_front().unwrap()];
            path.push(current.get_name().to_string());

            if self.nodes[current].get_ip() == end {
                return Some(path);
            }

            if let Some(adjacent) = self.adjacency.get(current.get_name()) {
                for node in adjacent {
                    if !visited.contains_key(&node as &str) {
                        visited.insert(node, true);
                        queue.push_back(node);
                    }
                }
            }
        }
        None
    }

    fn add_node(&mut self, new_node: dyn NetworkNode, adjacent: Vec<String>) {
        self.nodes.insert(new_node.get_name().clone(), new_node);
        self.adjacency.insert(new_node.get_name().clone(), adjacent);
    }

    pub fn print_topology(&self) {
        for (node, adjacent) in self.adjacency.iter() {
            println!("{}: {:?}", node, adjacent);
        }
    }
}

impl Topology {
    pub fn new() -> Topology {
        Topology {
            nodes: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn generate_topology_from_file(filename: String) -> Topology {
        let mut topology = Topology {
            nodes: HashMap::new(),
            adjacency: HashMap::new(),
        };

        let file = std::fs::read_to_string(filename).unwrap();
        let lines: Vec<&str> = file.split("\r").collect();
        for line in lines {
            let line: Vec<&str> = line.split(" ").collect();
            let node_name = line[0];
            let node_ip = line[1];
            let node_type = line[2];
            if node_type == "switch" {
                let number_of_ports = line[3].parse::<u16>().unwrap();
                let weights: Vec<u16> = line[4]
                    .split(" ")
                    .map(|x| x.trim().parse::<u16>().unwrap())
                    .collect();
                let new_switch = Switch::new(node_name, node_ip, number_of_ports, weights);
                let adjacent: Vec<String> = line[5]
                    .split(" ")
                    .map(|x| x.to_string())
                    .collect();
                topology.add_node(new_switch, adjacent);
                debug!("Added switch: {:?}", new_switch);
            } else {
                let new_server = Server::new(node_name, node_ip);
                let adjacent: Vec<String> = line[3]
                    .split(" ")
                    .map(|x| x.trim().to_string())
                    .collect();
                topology.add_node(new_server, adjacent);
                debug!("Added server: {:?}", new_server);
            }
        }
        topology
    }
}
