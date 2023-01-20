use std::collections::HashSet;

pub struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    tail_positions: HashSet<(i32, i32)>,
}

impl Rope {
    pub fn new(start: (i32, i32)) -> Self { 
        Self {
            head: start,
            tail: start,
            tail_positions: HashSet::from([start]),
        }
    }

    pub fn get_pos(&self) -> ((i32, i32), (i32, i32)) {
        (self.head, self.tail)
    }

    fn pull_once(&mut self, dir: (i32, i32)) {
        self.head = (self.head.0 + dir.0, self.head.1 + dir.1);
        let diff_x = (self.head.0 - self.tail.0).abs();
        let diff_y = (self.head.1 - self.tail.1).abs();
        if diff_x > 1 || diff_y > 1 {
            self.tail = (self.head.0 - dir.0, self.head.1 - dir.1);
            self.tail_positions.insert(self.tail);
        }
    }

    pub fn pull(&mut self, dir: (i32, i32), repeats: i32) {
        for _ in 0..repeats {
            self.pull_once(dir);
        }
    }

    pub fn get_tail_count(&self) -> i32 {
        self.tail_positions.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope() {
        let rope = Rope::new((4, 0));
        assert_eq!(rope.get_pos(), ((4, 0), (4, 0)));
    }

    #[test]
    fn test_rope_pull_right() {
        let mut rope = Rope::new((4, 0));
        
        rope.pull((0, 1), 1);
        assert_eq!(rope.get_pos(), ((4, 1), (4, 0)));

        rope.pull((0, 1), 1);
        assert_eq!(rope.get_pos(), ((4, 2), (4, 1)));

        rope.pull((0, 1), 1);
        assert_eq!(rope.get_pos(), ((4, 3), (4, 2)));

        rope.pull((0, 1), 1);
        assert_eq!(rope.get_pos(), ((4, 4), (4, 3)));
    }

    #[test]
    fn test_rope_pull_up() {
        let mut rope = Rope::new((4, 0));
        
        rope.pull((0, 1), 4);
        assert_eq!(rope.get_pos(), ((4, 4), (4, 3)));
        
        rope.pull((-1, 0), 1);
        assert_eq!(rope.get_pos(), ((3, 4), (4, 3)));
        
        rope.pull((-1, 0), 1);
        assert_eq!(rope.get_pos(), ((2, 4), (3, 4)));
        
        rope.pull((-1, 0), 1);
        assert_eq!(rope.get_pos(), ((1, 4), (2, 4)));
        
        rope.pull((-1, 0), 1);
        assert_eq!(rope.get_pos(), ((0, 4), (1, 4)));
    }

    #[test]
    fn test_rope_pull() {
        let mut rope = Rope::new((4, 0));
        // pull right 4
        rope.pull((0, 1), 4);
        assert_eq!(rope.get_pos(), ((4, 4), (4, 3)));

        // pull up 4
        rope.pull((-1, 0), 4);
        assert_eq!(rope.get_pos(), ((0, 4), (1, 4)));

        // pull left 3
        rope.pull((0, -1), 3);
        assert_eq!(rope.get_pos(), ((0, 1), (0, 2)));

        // pull down 1
        rope.pull((1, 0), 1);
        assert_eq!(rope.get_pos(), ((1, 1), (0, 2)));
    }
}
