
pub trait NetworkNode {
    fn get_name(&self) -> String;
    fn get_ip(&self) -> String;
}

#[derive(Debug)]
pub struct Server {
    pub name: String,
    pub ip: String,
}

impl Server {
    pub fn new(name: String, ip: String) -> Server {
        Server { name, ip }
    }
}

impl NetworkNode for Server {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_ip(&self) -> String {
        self.ip.clone()
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
    pub fn new(name: String, ip: String, number_of_ports: u16, weights: Vec<u16>) -> Switch {
        Switch {
            name,
            ip,
            number_of_ports,
            weights,
        }
    }
}
impl NetworkNode for Switch {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_ip(&self) -> String {
        self.ip.clone()
    }
}
