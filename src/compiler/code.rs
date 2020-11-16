use std::fmt::Display;
use std::fmt;

pub struct Code {
    lines: Vec<String>
}

impl Code {
    pub fn new() -> Self {
        Self {
            lines: vec![],
        }
    }

    pub fn from_line(line: &str) -> Self {
        let mut new = Self::new();
        new.add_line(line);
        new
    }

    pub fn merge(left: &mut Self, right: &mut Self) -> Self {
        let mut new = Self::new();
        new.append(left);
        new.append(right);
        new
    }

    pub fn add_line(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }

    pub fn add_lines(&mut self, lines: &[&str]) {
        for line in lines {
            self.add_line(line)
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.lines.append(other.lines.as_mut());
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.lines.join("\n");
        write!(f, "{}\n", code)
    }
}
