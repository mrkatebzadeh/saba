pub struct Connection {
    pub src: String,
    pub dst: String,
}

impl Connection {
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
}
