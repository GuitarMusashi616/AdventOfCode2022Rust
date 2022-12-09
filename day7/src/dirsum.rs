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

    fn capture_if_important(&self, path: &'a str, size: usize, sizes: &mut HashMap<&'a str, usize>) {
        // only save it if it is a directory and it is not already saved
        if !self.dirs.contains_key(path) || sizes.contains_key(path) {
            return;
        }

        sizes.insert(path, size);
    }

    pub fn get_all_sizes(&self, glob: &'a str, sizes: &mut HashMap<&'a str, usize>) -> usize {
        if let Some(size) = self.files.get(glob) {
            return *size;
        }
        // otherwise it's a directory
        let files = self.dirs.get(glob).expect("glob is not a directory or file");

        let mut total = 0;
        for file in files {
            let size = self.get_all_sizes(file, sizes);
            self.capture_if_important(file, size, sizes);
            total += size
        }
        self.capture_if_important(glob, total, sizes);
        total
    }

    pub fn get_size_of_lt_100000(&self) -> usize {
        self.dirs.iter().map(|(dir, _)| self.get_size(dir)).filter(|x| *x <= 100000).sum()
    }

    pub fn get_dir_to_delete(&self, min_space_needed: usize, total_space: usize) -> (&'a str, usize) {
        // sort the keys = nlogn
        // find the smallest greater than x o(n) (must find space need to delete first)
        // heapify O(n) + pops*logn

        let mut sizes = HashMap::new();
        self.get_all_sizes("/", &mut sizes);
        dbg!(&sizes);
        let used_space = sizes.get("/").expect("no root(/) directory");
        let space_available = total_space - used_space;
        let required_space_to_delete = min_space_needed - space_available;
        println!("{}/{}", used_space, total_space);
        println!("{}/{} required for update", space_available, min_space_needed);
        println!("need {} more space", required_space_to_delete);

        // find the smallest difference from space_req_to_del and sizeof dir
        let (dir, diff) = sizes.iter().fold(("None", usize::MAX), |mut acc, (name, size)| {
            if *size < required_space_to_delete {
                return acc;
            }
            let diff = size - required_space_to_delete;
            if diff < acc.1 {
                acc = (name, diff);
            }
            acc
        });

        (dir, diff+required_space_to_delete)
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

        let dirsum = DirSum::new(&exp);
        dbg!(&dirsum);

        let size = dirsum.get_size("/");
        assert_eq!(size, 48381165);

        let answer = dirsum.get_size_of_lt_100000();
        assert_eq!(answer, 95437);
    }

    #[test]
    fn test_get_all_sizes() {
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

        let dirsum = DirSum::new(&exp);
        let mut sizes = HashMap::new();
        dirsum.get_all_sizes("/", &mut sizes);

        let exp = HashMap::from([
            ("/", 48381165),
            ("/d", 24933642),
            ("/a", 94853),
            ("/a/e", 584),
        ]);

        assert_eq!(sizes, exp);
    }

    #[test]
    fn test_get_file_to_delete() {

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
        let (dir, size) = dirsum.get_dir_to_delete(30000000, 70000000);
        assert_eq!((dir, size), ("/d", 24933642));
    }
}
