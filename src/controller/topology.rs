use crate::node::{NetworkNode, Server, Switch};
use log::debug;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Topology {
    nodes: HashMap<String, Box<dyn NetworkNode>>,
    adjacency: HashMap<String, Vec<String>>,
}

impl Topology {
    #[allow(dead_code)]
    pub fn dfs(&self, start: &str, end: &str) -> Option<Vec<String>> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        let path = vec![String::from(start)];

        queue.push_back((start, path));
        visited.insert(start, true);

        debug!("Starting DFS from {} to {}", &start, &end);
        while !queue.is_empty() {
            let top = queue.pop_front().unwrap();
            let (current, path) = (&self.nodes[top.0], top.1);

            if current.get_name() == end {
                debug!("Found path: {:?}", path);
                return Some(path);
            }

            if let Some(adjacent) = self.adjacency.get(&current.get_name()) {
                for node in adjacent {
                    if !visited.contains_key(node as &str) {
                        visited.insert(node, true);
                        let mut new_path = path.clone();
                        new_path.push(node.to_string());
                        queue.push_back((node, new_path));
                    }
                }
            }
        }
        None
    }

    #[allow(dead_code)]
    fn add_server(&mut self, new_server: Server, adjacent: Vec<String>) {
        let name = new_server.get_name();
        self.nodes.insert(name.clone(), Box::new(new_server));
        self.adjacency.insert(name, adjacent);
    }

    #[allow(dead_code)]
    fn add_switch(&mut self, new_switch: Switch, adjacent: Vec<String>) {
        let name = new_switch.get_name();
        self.nodes.insert(name.clone(), Box::new(new_switch));
        self.adjacency.insert(name, adjacent);
    }
}

impl std::fmt::Display for Topology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (node, adjacent) in self.adjacency.iter() {
            write!(f, "{node}: {adjacent:?}")?
        }
        Ok(())
    }
}

impl Topology {
    #[allow(dead_code)]
    pub fn new() -> Topology {
        Topology {
            nodes: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn generate_topology_from_file(filename: String) -> Topology {
        let mut topology = Topology {
            nodes: HashMap::new(),
            adjacency: HashMap::new(),
        };

        let file = std::fs::read_to_string(filename).unwrap();
        let lines: Vec<&str> = file.split('\n').collect();
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let line: Vec<&str> = line.split(',').collect();
            let node_name = line[0].trim();
            let node_ip = line[1].trim();
            let node_type = line[2].trim();
            if node_type == "switch" {
                let number_of_ports = line[3].trim().parse::<u16>().unwrap();
                let weights: Vec<u16> = line[4]
                    .trim()
                    .split(' ')
                    .map(|x| x.trim().parse::<u16>().unwrap())
                    .collect();
                let new_switch = Switch::new(node_name, node_ip, number_of_ports, weights);
                let adjacent: Vec<String> = line[5].split(' ').map(|x| x.to_string()).collect();
                debug!("Added switch: {new_switch:?}");
                topology.add_switch(new_switch, adjacent);
            } else {
                let weights: Vec<u16> = line[3]
                    .trim()
                    .split(' ')
                    .map(|x| x.trim().parse::<u16>().unwrap())
                    .collect();
                let switch = line[4].trim();
                let new_server = Server::new(node_name, node_ip, weights);
                debug!("Added server: {new_server:?}");
                topology.add_server(new_server, vec![switch.to_string()]);
            }
        }
        topology
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let mut topology = Topology::new();
        let s1 = Server::new("s1", "127.0.0.1", vec![1, 2, 3]);
        let s2 = Server::new("s2", "127.0.0.2", vec![1, 2, 3]);
        let switch1 = Switch::new("switch1", "127.0.0.3", 3, vec![1, 2, 3]);
        topology.add_server(s1, vec!["switch1".to_string()]);
        topology.add_server(s2, vec!["switch1".to_string()]);

        topology.add_switch(switch1, vec!["s1".to_string(), "s2".to_string()]);
        assert!(topology.dfs("s1", "s2").unwrap() == vec!["s1", "switch1", "s2"]);
    }

    #[test]
    fn test_dfs_no_path() {
        let mut topology = Topology::new();
        let s1 = Server::new("s1", "127.0.0.1", vec![1, 2, 3]);
        let s2 = Server::new("s2", "127.0.0.2", vec![1, 2, 3]);
        let switch1 = Switch::new("switch1", "127.0.0.3", 3, vec![1, 2, 3]);
        topology.add_server(s1, vec!["switch1".to_string()]);
        topology.add_server(s2, vec![]);
        topology.add_switch(switch1, vec!["s1".to_string()]);
        assert!(topology.dfs("s1", "s2").is_none());
    }

    #[test]
    fn test_dfs_from_file() {
        let topology = Topology::generate_topology_from_file("tests/topology.csv".to_string());
        println!("{topology:?}");
        assert!(
            topology.dfs("switch1", "server1").unwrap() == vec!["switch1", "switch4", "server1"]
        );
    }
}
