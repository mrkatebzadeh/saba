pub struct Connection {
    pub src: String,
    pub dst: String,
}

impl Connection {
    pub fn new(src: String, dst: String) -> Self {
        Connection { src, dst }
    }
}

