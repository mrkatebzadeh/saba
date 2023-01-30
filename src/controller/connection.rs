#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Connection {
    pub src: String,
    pub dst: String,
}

impl Connection {
    #[allow(dead_code)]
    pub fn new(src: String, dst: String) -> Self {
        Connection { src, dst }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        let connection = Connection::new("a".to_string(), "b".to_string());
        assert_eq!(connection.src, "a");
        assert_eq!(connection.dst, "b");
    }

    #[test]
    fn test_connection_eq() {
        let connection1 = Connection::new("a".to_string(), "b".to_string());
        let connection2 = Connection::new("a".to_string(), "b".to_string());
        assert_eq!(connection1, connection2);
    }

    #[test]
    fn test_connection_neq() {
        let connection1 = Connection::new("a".to_string(), "b".to_string());
        let connection2 = Connection::new("a".to_string(), "c".to_string());
        assert_ne!(connection1, connection2);
    }
}
