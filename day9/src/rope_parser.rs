use std::fs;

use crate::rope::Rope;

pub struct RopeParser {
    rope: Rope,
}

impl RopeParser {
    pub fn new(rope: Rope) -> Self {
        Self { rope }
    }

    pub fn parse(&mut self, string: &str) {
        for line in string.lines() {
            self.parse_line(line);
        }
    }

    pub fn parse_line(&mut self, line: &str) {
        if line.len() <= 1 {
            return;
            // panic!("line is invalid {}", line);
        }
        let mut strings = line.split_whitespace();
        let dir = match strings.next().expect("line must contain string") {
            "R" => (0, 1),
            "L" => (0, -1),
            "U" => (-1, 0),
            "D" => (1, 0),
            _ => panic!("could not parse line {}", line),
        };

        let repeats: i32 = strings
            .next()
            .expect("line must have number")
            .parse()
            .expect("2nd contiguous string in line should be a number");

        self.rope.pull(dir, repeats)
    }

    pub fn parse_file(&mut self, filename: &str) {
        let string = fs::read_to_string(filename).expect("file not found");
        self.parse(&string);
    }

    pub fn get_pos(&self) -> ((i32, i32), (i32, i32)) {
        self.rope.get_pos()
    }

    pub fn get_tail_count(&self) -> i32 {
        self.rope.get_tail_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> RopeParser {
        let rope = Rope::new((4, 0));
        RopeParser::new(rope)
    }

    #[test]
    fn test_parse_line() {
        let mut rp = fixture();
        rp.parse_line("R 4");
        assert_eq!(rp.get_pos(), ((4, 4), (4, 3)));
    }

    #[test]
    fn test_parse_string() {
        let mut rp = fixture();
        rp.parse("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2");
        assert_eq!(rp.get_tail_count(), 13);
    }

    #[test]
    fn test_tail_pos() {
        let mut rp = fixture();
        rp.parse("R 4\nU 4");
        assert_eq!(rp.get_pos(), ((0, 4), (1, 4)));
    }
}
