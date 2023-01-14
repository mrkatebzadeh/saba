use crate::node::{NetworkNode, Server, Switch};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Topology {
    nodes: HashMap<String, Box<dyn NetworkNode>>,
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
            let current = &self.nodes[queue.pop_front().unwrap()];
            path.push(current.get_name().to_string());

            if current.get_ip() == end {
                return Some(path);
            }

            if let Some(adjacent) = self.adjacency.get(&current.get_name()) {
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

    fn add_server(&mut self, new_server: Server, adjacent: Vec<String>) {
        let name = new_server.get_name();
        self.nodes.insert(name.clone(), Box::new(new_server));
        self.adjacency.insert(name, adjacent);
    }

    fn add_switch(&mut self, new_switch: Switch, adjacent: Vec<String>) {
        let name = new_switch.get_name();
        self.nodes.insert(name.clone(), Box::new(new_switch));
        self.adjacency.insert(name, adjacent);
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
        let lines: Vec<&str> = file.split("\n").collect();
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let line: Vec<&str> = line.split(",").collect();
            let node_name = line[0].trim();
            let node_ip = line[1].trim();
            let node_type = line[2].trim();
            if node_type == "switch" {
                let number_of_ports = line[3].trim().parse::<u16>().unwrap();
                let weights: Vec<u16> = line[4]
                    .trim()
                    .split(" ")
                    .map(|x| x.trim().parse::<u16>().unwrap())
                    .collect();
                let new_switch = Switch::new(node_name, node_ip, number_of_ports, weights);
                let adjacent: Vec<String> = line[5].split(" ").map(|x| x.to_string()).collect();
                debug!("Added switch: {:?}", new_switch);
                topology.add_switch(new_switch, adjacent);
            } else {
                let weights: Vec<u16> = line[3]
                    .trim()
                    .split(" ")
                    .map(|x| x.trim().parse::<u16>().unwrap())
                    .collect();
                let switch = line[4].trim();
                let new_server = Server::new(node_name, node_ip, weights);
                debug!("Added server: {:?}", new_server);
                topology.add_server(new_server, vec![switch.to_string()]);
            }
        }
        topology
    }
}
