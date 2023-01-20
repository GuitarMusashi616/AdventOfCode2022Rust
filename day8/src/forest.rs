use std::fmt::{Display, Debug};

const ZERO: u8 = 48;
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tree {
    Visible,
    Invisible,
}

pub fn get_dim(string: &str) {
    let mut N = 0;
    let mut M = 0;

    for line in string.lines() {
        if N == 0 {
            N = line.len()
        }
        M += 1;
    }
    println!("N: {}, M: {}", N, M);
}

pub struct Forest<const N: usize, const M: usize> {
    grid: [[u8; N]; M],
    visible: [[Tree; N]; M],
}

impl<const N: usize, const M: usize> Forest<N, M> {
    pub fn new(grid: [[u8; N]; M]) -> Self {
        Forest {
            grid,
            visible: [[Tree::Invisible; N]; M],
        }
    }

    pub fn to_str(&self) -> String {
        let mut string = String::new();
        for row in self.grid {
            for col in row {
                let digit = col + ZERO;
                string.push(digit as char);
            }
            string.push('\n')
        }
        string
    }

    pub fn from_str(string: &str) -> Self {
        let mut grid = [[0; N]; M];
        for (i, line ) in string.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                let ascii = char as u8;
                grid[i][j] = ascii - ZERO;
            }
        }
        // println!("{:?}", grid);
        Forest::<N, M>::new(grid)
    }

    pub fn visibility_to_str(&self) -> String {
        let mut string = String::new();
        for row in self.visible {
            for col in row {
                let repr = match col {
                    Tree::Visible => 'V',
                    Tree::Invisible => 'I',
                };
                string.push(repr);
            }
            string.push('\n')
        }
        string
    }

    fn in_bounds(&self, i: isize, j: isize) -> bool {
        if (0 <= i && i < N as isize) && (0 <= j && j < M as isize) {
            return true;
        }
        false
    }

    pub fn clearing_to_edge(&self, mut i: isize, mut j: isize, dir: (isize, isize)) -> bool {
        let (dir_i, dir_j) = dir;
        let this_tree_height = self.grid[i as usize][j as usize];
        loop {
            i += dir_i;
            j += dir_j;
            if !self.in_bounds(i, j) {
                break;
            }
            let that_tree_height = self.grid[i as usize][j as usize];
            if this_tree_height <= that_tree_height {
                return false;
            }
        }
        true
    }

    pub fn count_trees_visible_in_dir(&self, mut i: isize, mut j: isize, dir: (isize, isize)) -> u32 {
        let (dir_i, dir_j) = dir;
        let this_tree_height = self.grid[i as usize][j as usize];
        let mut vis_count = 0;
        loop {
            i += dir_i;
            j += dir_j;
            if !self.in_bounds(i, j) {
                break;
            }
            let that_tree_height = self.grid[i as usize][j as usize];
            if this_tree_height <= that_tree_height {
                return vis_count + 1;
            }
            vis_count += 1;
        }
        vis_count
    }

    pub fn mark_visible(&mut self) {
        for i in 0..N {
            for j in 0..M {
                for dir in DIRECTIONS {
                    let visible = self.clearing_to_edge(i as isize, j as isize, dir);
                    if visible {
                        self.visible[i][j] = Tree::Visible;
                        break;
                    }
                }
            }
        }
    }

    pub fn count_visible(&self) -> u32 {
        let mut count = 0;
        for row in self.visible {
            for col in row {
                if col == Tree::Visible {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn find_max_scenic_score(&mut self) -> u32 {
        let mut highest = 0;
        for i in 0..N {
            for j in 0..M {
                let mut scenic_counts = Vec::new();
                for dir in DIRECTIONS {
                    let vis_count = self.count_trees_visible_in_dir(i as isize, j as isize, dir);
                    scenic_counts.push(vis_count);
                }
                let scenic_score = scenic_counts.into_iter().fold(1, |acc, x| acc*x);
                if scenic_score > highest {
                    highest = scenic_score;
                }
            }
        }
        highest
    }

}

impl<const N: usize, const M: usize> Display for Forest<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl<const N: usize, const M: usize> Debug for Forest<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.to_str(), self.visibility_to_str())
    }
}

impl<const N: usize, const M: usize> From<&str> for Forest<N, M> {
    fn from(input: &str) -> Self {
        Self::from_str(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_fixture() -> Forest<5, 5> {
        Forest::<5, 5>::new(
            [
                [3, 0, 3, 7, 3],
                [2, 5, 5, 1, 2],
                [6, 5, 3, 3, 2],
                [3, 3, 5, 4, 9],
                [3, 5, 3, 9, 0],
            ]
        )
    }

    #[test]
    fn test_new_forest_to_str() {
        let forest = default_fixture();
        assert_eq!(forest.to_str(), "30373\n25512\n65332\n33549\n35390\n".to_owned());
    }

    #[test]
    fn test_clearing_to_edge() {
        let forest = default_fixture();
        let exp = [
            [true, true, true, true, true],
            [true, true, true, false, true],
            [true, true, false, true, true],
            [true, false, true, false, true],
            [true, true, true, true, true],
        ];
        let mut res = [[false; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                for dir in DIRECTIONS {
                    let visible = forest.clearing_to_edge(i as isize, j as isize, dir);
                    if visible {
                        res[i][j] = true;
                        break;
                    }
                }
            }
        }
        assert_eq!(res, exp);
    }

    #[test]
    fn test_mark_visible() {
        let mut forest = default_fixture();
        forest.mark_visible();
        assert_eq!(forest.visibility_to_str(), "VVVVV\nVVVIV\nVVIVV\nVIVIV\nVVVVV\n".to_owned());
    }

    #[test]
    fn test_max_scenic_score() {
        let mut forest = default_fixture();
        let res = forest.find_max_scenic_score();
        assert_eq!(res, 8);
    }
}