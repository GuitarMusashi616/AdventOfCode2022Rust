// functions for adding up the size of directories in hash map
use std::collections::HashMap;
use crate::lineparser::Glob;
use crate::lineparser::Parser;

#[derive(Debug)]
pub struct DirSum<'a> {
    dirs: HashMap<&'a str, Vec<&'a str>>,
    files: HashMap<&'a str, usize>,
}

impl<'a> DirSum<'a> {
    pub fn new(parser_data: &'a HashMap<String, Glob>) -> Self {
        let (dirs, files) = Self::parser_data_to_dirs_and_files(parser_data);

        Self {
            dirs,
            files,
        }
    }

    fn parser_data_to_dirs_and_files(parser_data: &'a HashMap<String, Glob>) -> (HashMap<&'a str, Vec<&'a str>>, HashMap<&'a str, usize>) {
        let mut dirs: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
        let mut files: HashMap<&'a str, usize> = HashMap::new();

        for (k,v) in parser_data {

            let parent = Self::get_prev_path(k);
            let parent_entry = dirs.entry(parent).or_default();

            match v {
                Glob::File(size) => {
                    files.insert(k, *size);
                    parent_entry.push(k);
                },
                Glob::Dir => {
                    parent_entry.push(k);
                },
            }
        }

        (dirs, files)
    }

    fn get_prev_path(k: &str) -> &str {
        let mut index = k.chars().rev().enumerate().find(|(_, x)| *x == '/').map(|(i, _)| k.len() - i - 1).unwrap();
        if index == 0 {index += 1};
        k.get(0..index).unwrap()
    }

    pub fn get_size(&self, glob: &str) -> usize {
        if let Some(size) = self.files.get(glob) {
            return *size;
        }
        // otherwise it's a directory
        let files = self.dirs.get(glob).expect("glob is not a directory or file");

        let mut total = 0;
        for file in files {
            let size = self.get_size(file);
            total += size
        }
        total
    }

    pub fn get_size_of_lt_100000(&self) -> usize {
        self.dirs.iter().map(|(dir, _)| self.get_size(dir)).filter(|x| *x <= 100000).sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirsum() {

        let exp: HashMap<String, Glob> = HashMap::from([
            ("/a".into(), Glob::Dir),
            ("/b.txt".into(), Glob::File(14848514)),
            ("/c.dat".into(), Glob::File(8504156)),
            ("/d".into(), Glob::Dir),
            ("/a/e".into(), Glob::Dir),
            ("/a/f".into(), Glob::File(29116)),
            ("/a/g".into(), Glob::File(2557)),
            ("/a/h.lst".into(), Glob::File(62596)),
            ("/a/e/i".into(), Glob::File(584)),
            ("/d/j".into(), Glob::File(4060174)),
            ("/d/d.log".into(), Glob::File(8033020)),
            ("/d/d.ext".into(), Glob::File(5626152)),
            ("/d/k".into(), Glob::File(7214296)),
        ]);

        let mut dirsum = DirSum::new(&exp);
        dbg!(&dirsum);

        let size = dirsum.get_size("/");
        assert_eq!(size, 48381165);

        let answer = dirsum.get_size_of_lt_100000();
        assert_eq!(answer, 95437);


        // for (k,v) in exp {
        //     let mut index = k.chars().rev().enumerate().find(|(_, x)| *x == '/').map(|(i, _)| k.len() - i - 1).unwrap();
        //     if index == 0 {index += 1};
        //     let prev_dir = k.get(0..index).unwrap();

        //     println!("prev_dir of {} => {}", k, prev_dir);
        //     if !sizes.contains_key(prev_dir) {
        //         sizes.insert(prev_dir.into(), 0);
        //     } 

        //     let cursize = sizes.get(prev_dir).unwrap();

        // }

        // for every dir recursion - figur
    }
}
