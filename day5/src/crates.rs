use std::fmt;

use crate::parse_commands;


#[macro_export]
macro_rules! crates {
    [ $( [ $( $d:expr ),* ] ),* ] => {
        Crates::from(vec![
            $(
                vec![$($d),*],
            )* 
        ])
    }
}

#[derive(Debug)]
pub struct Crates {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
pub enum CrateErr {
    PopFromEmpty,
}

impl Crates {
    pub fn new(size: usize) -> Self {
        Self {
            stacks: vec![vec![]; size],
        }
    }

    pub fn move_from(&mut self, count: usize, a: usize, b: usize) -> Result<(), CrateErr> {
        for _ in 0..count {
            match self.stacks[a-1].pop() {
                Some(val) => self.stacks[b-1].push(val),
                None => return Err(CrateErr::PopFromEmpty),
            };
        }

        Ok(())
    }

    pub fn move_ordered(&mut self, count: usize, a: usize, b: usize) -> Result<(), CrateErr> {
        let mut temp = Vec::new();
        for _ in 0..count {
            match self.stacks[a-1].pop() {
                Some(val) => temp.push(val),
                None => return Err(CrateErr::PopFromEmpty),
            };
        }

        for val in temp.into_iter().rev() {
            self.stacks[b-1].push(val)
        }

        Ok(())
    }

    pub fn execute(&mut self, content: &str) -> Result<String, CrateErr> {
        for [count, from, to] in parse_commands(content) {
            self.move_from(count, from, to)?;
        }

        Ok(self.to_string())
    }

    pub fn execute_ordered(&mut self, content: &str) -> Result<String, CrateErr> {
        for [count, from, to] in parse_commands(content) {
            self.move_ordered(count, from, to)?;
        }

        Ok(self.to_string())
    }

    pub fn get_stack_tops(&self) -> String {
        let mut output = String::new();
        for stack in &self.stacks {
            assert!(!stack.is_empty());
            output.push(stack[stack.len()-1]);
        }
        output
    }
}

impl<const N: usize> From<[&str; N]> for Crates {
    fn from(input: [&str; N]) -> Self {
        let mut stacks = Vec::new();
        for line in input {
            let inner = line.chars().collect();
            stacks.push(inner)
        }

        Self {
            stacks,
        }
    }
}

impl From<Vec<Vec<char>>> for Crates {
    fn from(vec: Vec<Vec<char>>) -> Self {
        Self {
            stacks: vec,
        }
    }
}

impl fmt::Display for Crates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_stack_tops())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let res = crates![
            ['Z', 'N'],
            ['M','C','D'],
            ['P']
        ];
        dbg!(res);
    }
}

