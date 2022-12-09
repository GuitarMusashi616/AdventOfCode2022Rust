// so how should we parse this
// every dir should cd "a" should hashmap["a"] to return node?
// a bunch of hashmaps within hashmaps?
// cd /? make this the outer directory?
// ls show everything in the directory
//
// parse by each separate command and args
// ls -> command, args -> store each in a path

use std::{str::Lines, rc::Rc, collections::HashMap, num::ParseIntError};


pub struct Parser<'a> {
    // heap allocated trait?
    // should the heap allocated Iter give references to str? or just a whole String
    // well the entire &str is large, the lines() method returns &str, &String just derefs to &str

    iter: Lines<'a>,
    explorer: HashMap<&'a str, Glob<'a>>,
    curpath: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseErr {
    NoMoreLines,
    CommandNotRecognized,
    LineTooShort,
    NoPrevPath,
    PathDoesNotExit,
    NoPathArg,
    ParseIntError,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Glob<'a> {
    File(usize),
    Dir(Vec<&'a str>)
}

impl From<ParseIntError> for ParseErr {
    fn from(err: ParseIntError) -> ParseErr {
        ParseErr::ParseIntError
    }
}

// every path is unique
// each path will be either a file or dir
// File will have a size
// dir will have a vec of strings
// the dir strings will all be temporary? getting the strings directly from the &str content input,
// therefore it must live as long as the original which lives as long as ''

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            iter: content.lines(),
            explorer: HashMap::new(),
            curpath: "/".into(),
        }
    }

    pub fn next_command(&mut self) -> Result<(), ParseErr> {
        let line = Self::grab_next_line(&mut self.iter)?;

        match line.get(0..3) {
            Some("$ cd") => {
                let path = line.get(5..).ok_or(ParseErr::LineTooShort)?;
                self.cd(path)?;
            },
            Some("$ ls") => {
                self.ls()?;
            },
            Some(_) => (),
            None => (),
        };

        dbg!(line);
        Ok(())
    }

    fn pops_to_return_to_prev_path(path: &str) -> Result<usize, ParseErr> {
        // pop()s required to turn (root/value/thing => root/value) in this case 6
        if path == "/" {
            return Err(ParseErr::NoPrevPath);
        }
        path.chars().rev().enumerate().find(|(_, chr)| *chr == '/').map_or(Err(ParseErr::NoPrevPath), |(i, _)| Ok(i + 1))
    }

    fn set_curpath_to_prev(&mut self) -> Result<(), ParseErr> {
        // reverse iterate until / is found
        // return substring up to /

        let index = Self::pops_to_return_to_prev_path(&self.curpath)?;

        if index > self.curpath.len() {
            return Err(ParseErr::LineTooShort);
        }

        (0..index).for_each(|_| {self.curpath.pop();});

        println!("the / in the path is at {}, the string up to that point is {}", index, &self.curpath);

        Ok(())
    }

    pub fn cd(&mut self, directory: &str) -> Result<(), ParseErr> {
        // take the directory and change the current path?

        // case of /, change directory to outermost
        if directory == "/" {
            self.curpath = "/".into();
            return Ok(());
        }

        // case of .., change directory to previous
        if directory == ".." {
            // remove the last /path ie /root/file => /root
            self.set_curpath_to_prev()?;
        }

        if !self.explorer.contains_key(directory) {
            return Err(ParseErr::PathDoesNotExit);
        }

        self.curpath.push_str(directory);

        Ok(())
    }

    fn grab_next_line(lines: &mut Lines<'a>) -> Result<&'a str, ParseErr> {
        Ok(lines.next().ok_or(ParseErr::NoMoreLines)?.trim())
    }

    pub fn ls(&mut self) -> Result<(), ParseErr> {
        let mut line = Self::grab_next_line(&mut self.iter)?;
        while line.get(0..1) != Some("$") {
            if line.get(0..3) == Some("dir") {
                let dir = line.get(4..).ok_or(ParseErr::NoPathArg)?;
                self.save_dir_to_path(dir);
            } else {
                let mut words = line.split_whitespace();
                let size = words.next().ok_or(ParseErr::NoPathArg)?.parse::<usize>()?;
                let file = words.next().ok_or(ParseErr::NoPathArg)?;
                self.save_file_to_path(file, size);
            }
            line = Self::grab_next_line(&mut self.iter)?;
        }

        Ok(())
    }

    fn save_dir_to_path(&mut self, path: &'a str) {
        self.explorer.insert(path, Glob::Dir(Vec::new()));
    }

    fn save_file_to_path(&mut self, path: &'a str, size: usize) {
        self.explorer.insert(path, Glob::File(size));
    }

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

        let mut parser = Parser::new(content);
        parser.next_command().unwrap();
        assert_eq!(parser.curpath, "/".to_owned());

        parser.next_command().unwrap();
        // move iterator up like 4
        // add each to the explorer
        let exp = HashMap::from([
            ("a", Glob::Dir(vec![])),
            ("b.txt", Glob::File(14848514)),
            ("c.dat", Glob::File(8504156)),
            ("d", Glob::Dir(vec![])),
        ]);
        assert_eq!(parser.explorer, exp);

        parser.next_command().unwrap();
        // assert_eq!(parser.curpath, "/a".to_owned());

    }

    #[test]
    fn test_path_back() {
        let mut parser = Parser::new("");

        parser.curpath = "/path/to/value/root".into(); 
        parser.set_curpath_to_prev().unwrap();
        let exp = "/path/to/value";
        assert_eq!(parser.curpath, exp);


        parser.curpath = "/".into(); 
        let exp = ParseErr::NoPrevPath;
        let res = parser.set_curpath_to_prev();
        assert_eq!(res.unwrap_err(), exp);
    }

    #[test]
    fn test_first_cd() {

    }
}
