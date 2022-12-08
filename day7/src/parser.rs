use crate::blob::Blob;
use std::{collections::HashMap, iter::Map, str::Lines};

// parser style eat the iter little by little
pub enum ParseErr {
    NoLinesRem,
}

pub struct Parser<'a> {
    lines: Lines<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            lines: content.lines(),
        }
    }

    pub fn process_command(&mut self) -> Result<(), ParseErr> {
        let line = self.lines.next().ok_or(ParseErr::NoLinesRem)?.trim();

        match line.get(0..5).unwrap() {
            "$ cd" => {}
            _ => (),
        };

        Ok(())
    }
}

pub fn parse(content: &str) -> Blob {
    let mut curpath = Vec::new();
    let mut children = Vec::new();

    for line in content.lines().map(|x| x.trim()) {
        if line.get(0..1) == Some("$") {
            match line.get(2..3) {
                Some("ls") => {
                    children = Vec::new();
                }
                Some("cd") => match line.get(5..) {
                    Some("..") => {
                        curpath.pop();
                    }
                    Some(path) => {
                        curpath.push(path);
                    }
                    _ => panic!("invalid path"),
                },
                _ => (),
            };
        } else {
            match line.get(0..3) {
                Some("dir") => {
                    let name = line.get(4..).unwrap().into();
                    children.push(Blob::Dir(name, Vec::new()));
                }
                _ => (),
            }
        }
        println!("{}", line);
    }
    Blob::File("doesn't".into(), 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_parse() {
        let content = "$ cd /
                       $ ls
                       dir a
                       14848514 b.txt
                       8504156 c.dat
                       dir d
                       $ cd a
                       $ ls
                       dir e
                       29116 f
                       2557 g
                       62596 h.lst
                       $ cd e
                       $ ls
                       584 i
                       $ cd ..
                       $ cd ..
                       $ cd d
                       $ ls
                       4060174 j
                       8033020 d.log
                       5626152 d.ext
                       7214296 k";

        parse(content);
    }
}
