use crate::connection::Connection;
use std::{collections::HashMap, fmt::Debug};

pub trait Enforcer: Debug {
    fn enforce(&mut self);
}

#[derive(Debug)]
struct SabaEnforcer {
    priority_levels: u8,

    //app,src,dst,bw,priority
    allocation_table: Vec<AllocationRecord>,

    priority_to_app_table: HashMap<u8, Vec<String>>,
    connection_to_app_table: HashMap<Connection, String>,
}

impl Enforcer for SabaEnforcer {
    fn enforce(&mut self) {
        unimplemented!()
    }
}

impl SabaEnforcer {
    #[allow(dead_code)]
    fn priority_to_app(&self, priority: u8) -> Option<&Vec<String>> {
        self.priority_to_app_table.get(&priority)
    }

    #[allow(dead_code)]
    fn connection_to_app(&self, connection: &Connection) -> Option<&String> {
        self.connection_to_app_table.get(connection)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct AllocationRecord {
    app: String,
    src: String,
    dst: String,
    bw: f32,
    priority: u8,
}

impl AllocationRecord {
    #[allow(dead_code)]
    fn new(app: String, src: String, dst: String, bw: f32, priority: u8) -> Self {
        AllocationRecord {
            app,
            src,
            dst,
            bw,
            priority,
        }
    }
}
