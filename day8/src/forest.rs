use std::fmt::{Display, Debug};

const ZERO: u8 = 48;
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tree {
    Visible,
    Invisible,
    Undetermined,
}

pub struct Forest<const N: usize, const M: usize> {
    grid: [[u8; N]; M],
    visible: [[Tree; N]; M],
}

impl<const N: usize, const M: usize> Forest<N, M> {
    pub fn new(grid: [[u8; N]; M]) -> Self {
        Forest {
            grid,
            visible: [[Tree::Undetermined; N]; M],
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

    pub fn visibility_to_str(&self) -> String {
        let mut string = String::new();
        for row in self.visible {
            for col in row {
                let repr = match col {
                    Tree::Visible => 'V',
                    Tree::Invisible => 'I',
                    Tree::Undetermined => 'U',
                };
                string.push(repr);
            }
            string.push('\n')
        }
        string
    }


    pub fn mark_edges_visible(&mut self) {
        for j in 0..M-1 {
            self.visible[0][j] = Tree::Visible;
        }
        for i in 0..N-1 {
            self.visible[i][N - 1] = Tree::Visible;
        }
        for j in (1..M).rev() {
            self.visible[M - 1][j] = Tree::Visible;
        }
        for i in (1..N).rev() {
            self.visible[i][0] = Tree::Visible;
        }
    }
    pub fn collect_adj_to_be_visible(&mut self, buffer: &mut Vec<(usize, usize)>, i: usize, j: usize) {
        for (di, dj) in DIRECTIONS {
            let (ni, nj) = (i as isize + di, j as isize + dj);
            // skip if out of bounds
            if (0 > ni || ni >= N as isize) || (0 > nj || nj >= M as isize) {
                continue
            }
            let ni = ni as usize;
            let nj = nj as usize;

            // skip if tree is already visible
            if self.visible[ni][nj] == Tree::Visible {
                continue;
            }

            let visible_tree = self.grid[i][j];
            let new_tree = self.grid[ni][nj];

            if new_tree > visible_tree {
                buffer.push((ni, nj))
            }
           
        }
    }

    pub fn get_cells_to_make_visible(&mut self) -> Vec<(usize, usize)> {
        let mut buffer = Vec::new();
        for i in 0..self.visible.len() {
            for j in 0..self.visible.len() {
                if self.visible[i][j] != Tree::Visible {
                    continue
                }
                self.collect_adj_to_be_visible(&mut buffer, i, j);
            }
        }
        buffer
    }

    pub fn mark_visible_neighbors(&mut self) -> usize {
        let cells_to_make_visible = self.get_cells_to_make_visible();
        let len = cells_to_make_visible.len();
        for (i, j) in cells_to_make_visible {
            self.visible[i][j] = Tree::Visible;
        }
        len
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
    fn test_mark_edges_visible() {
        let mut forest = default_fixture();
        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(forest.visible[i][j], Tree::Undetermined);
            }
        }
        forest.mark_edges_visible();
        for i in 0..5 {
            for j in [0, 4] {
                assert_eq!(forest.visible[i][j], Tree::Visible);
            }
        }
    }
    
    #[test]
    fn test_mark_visible_neighbors() {
        let mut forest = default_fixture();
        forest.mark_edges_visible();
        println!("{:?}", forest);
        loop {
            if forest.mark_visible_neighbors() <= 0 {
                break;
            }
            println!("{:?}", forest);
        }
    }

    #[test]
    fn test_mark_undetermined_neighbors_of_visible_to_visible_or_invisible() {
        // get the visible trees
        // get their undetermined neighbors
        // change to visible or invisible
    }
}