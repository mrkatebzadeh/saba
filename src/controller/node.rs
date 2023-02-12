use std::fmt::Debug;

pub trait NetworkNode {
    fn name(&self) -> String;
    fn ip(&self) -> String;
    fn weights(&self) -> Vec<u16>;
}

impl Debug for dyn NetworkNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NetworkNode{{{}}}", self.name())
    }
}

#[derive(Debug)]
pub struct Server {
    pub name: String,
    pub ip: String,
    pub weights: Vec<u16>,
}

impl Server {
    #[allow(dead_code)]
    pub fn new(name: &str, ip: &str, weights: Vec<u16>) -> Server {
        let name = name.to_string();
        let ip = ip.to_string();
        Server { name, ip, weights }
    }
}

impl NetworkNode for Server {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn ip(&self) -> String {
        self.ip.clone()
    }

    fn weights(&self) -> Vec<u16> {
        self.weights.clone()
    }
}

#[derive(Debug)]
pub struct Switch {
    pub name: String,
    pub ip: String,
    pub number_of_ports: u16,
    pub weights: Vec<u16>,
}

impl Switch {
    #[allow(dead_code)]
    pub fn new(name: &str, ip: &str, number_of_ports: u16, weights: Vec<u16>) -> Switch {
        let name = String::from(name);
        let ip = String::from(ip);
        Switch {
            name,
            ip,
            number_of_ports,
            weights,
        }
    }
}
impl NetworkNode for Switch {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn ip(&self) -> String {
        self.ip.clone()
    }

    fn weights(&self) -> Vec<u16> {
        self.weights.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server() {
        let server = Server::new("server1", "127.0.0.1", vec![1, 2, 3]);
        assert_eq!(server.name(), "server1");
        assert_eq!(server.ip(), "127.0.0.1");
        assert_eq!(server.weights(), vec![1, 2, 3]);
    }

    #[test]
    fn test_switch() {
        let switch = Switch::new("switch1", "127.0.0.1", 4, vec![1, 2, 3]);
        assert_eq!(switch.name(), "switch1");
        assert_eq!(switch.ip(), "127.0.0.1");
        assert_eq!(switch.weights(), vec![1, 2, 3]);
    }
}
