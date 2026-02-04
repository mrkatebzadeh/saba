/* topology.rs

*
* Author: M.R.Siavash Katebzadeh <mr@katebzadeh.xyz>
* Keywords: Rust
* Version: 0.0.1
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use crate::node::{NetworkNode, Server, Switch};
use log::debug;
use std::collections::{HashMap, VecDeque};
use std::path::Path;

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
        while let Some((node_name, path)) = queue.pop_front() {
            let current = &self.nodes[node_name];
            let current_name = current.name();

            if current_name == end {
                debug!("Found path: {:?}", path);
                return Some(path);
            }

            if let Some(adjacent) = self.adjacency.get(current_name) {
                for neighbor in adjacent {
                    let neighbor_name = neighbor.as_str();
                    if visited.contains_key(neighbor_name) {
                        continue;
                    }
                    visited.insert(neighbor_name, true);
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    queue.push_back((neighbor_name, new_path));
                }
            }
        }
        None
    }

    #[allow(dead_code)]
    fn add_server(&mut self, new_server: Server, adjacent: Vec<String>) {
        let name = new_server.name().to_owned();
        self.nodes.insert(name.clone(), Box::new(new_server));
        self.adjacency.insert(name, adjacent);
    }

    #[allow(dead_code)]
    fn add_switch(&mut self, new_switch: Switch, adjacent: Vec<String>) {
        let name = new_switch.name().to_owned();
        self.nodes.insert(name.clone(), Box::new(new_switch));
        self.adjacency.insert(name, adjacent);
    }

    fn parse_weights(segment: &str) -> Vec<u16> {
        segment
            .split_whitespace()
            .filter(|token| !token.is_empty())
            .map(|token| {
                token
                    .parse::<u16>()
                    .unwrap_or_else(|_| panic!("invalid weight '{token}' in topology"))
            })
            .collect()
    }

    fn parse_adjacent(segment: &str) -> Vec<String> {
        segment
            .split_whitespace()
            .filter(|token| !token.is_empty())
            .map(|token| token.to_string())
            .collect()
    }

    #[allow(dead_code)]
    pub fn new() -> Topology {
        Topology {
            nodes: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn generate_topology_from_file<P: AsRef<Path>>(filename: P) -> Topology {
        let mut topology = Topology::new();
        let path = filename.as_ref();
        let file = std::fs::read_to_string(path)
            .unwrap_or_else(|err| panic!("failed to read topology file {}: {err}", path.display()));

        for raw_line in file.lines() {
            let line = raw_line.trim();
            if line.is_empty() {
                continue;
            }

            let mut parts = line.splitn(6, ',');
            let node_name = parts
                .next()
                .and_then(|part| {
                    let trimmed = part.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed)
                    }
                })
                .expect("topology entry missing node name");
            let node_ip = parts
                .next()
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .expect("topology entry missing node IP");
            let node_type = parts
                .next()
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .expect("topology entry missing node type");

            if node_type == "switch" {
                let number_of_ports = parts
                    .next()
                    .map(|part| part.trim())
                    .filter(|part| !part.is_empty())
                    .expect("switch entry missing port count")
                    .parse::<u16>()
                    .expect("invalid port count in topology");
                let weights_segment = parts.next().map(|part| part.trim()).unwrap_or("");
                let adjacent_segment = parts.next().map(|part| part.trim()).unwrap_or("");

                let weights = Self::parse_weights(weights_segment);
                let adjacent = Self::parse_adjacent(adjacent_segment);

                let new_switch = Switch::new(node_name, node_ip, number_of_ports, weights);
                let switch_name = new_switch.name();
                let switch_ip = new_switch.ip();
                let switch_weights = new_switch.weights();
                debug!(
                    "Added switch {} (ip: {}, ports: {}, weights: {:?})",
                    switch_name, switch_ip, new_switch.number_of_ports, switch_weights
                );
                topology.add_switch(new_switch, adjacent);
            } else {
                let weights_segment = parts.next().map(|part| part.trim()).unwrap_or("");
                let adjacent_segment = parts.next().map(|part| part.trim()).unwrap_or("");
                let weights = Self::parse_weights(weights_segment);
                let adjacent = Self::parse_adjacent(adjacent_segment);

                let new_server = Server::new(node_name, node_ip, weights);
                let server_name = new_server.name();
                let server_ip = new_server.ip();
                let server_weights = new_server.weights();
                debug!(
                    "Added server {} (ip: {}, weights: {:?})",
                    server_name, server_ip, server_weights
                );
                topology.add_server(new_server, adjacent);
            }
        }
        topology
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
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/topology.csv");
        let topology = Topology::generate_topology_from_file(&path);
        println!("{topology:?}");
        assert!(
            topology.dfs("switch1", "server1").unwrap() == vec!["switch1", "switch4", "server1"]
        );
    }
}

/* topology.rs ends here */
