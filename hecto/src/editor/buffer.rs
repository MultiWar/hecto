pub struct Buffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        let mut lines: Vec<String> = Vec::new();
        lines.push(String::from("Hello, world!"));
        lines.push(String::from("Hello, world!"));
        Buffer { lines: lines }
    }
}
