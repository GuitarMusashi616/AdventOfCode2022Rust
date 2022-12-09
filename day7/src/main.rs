mod blob;
mod parser;
mod lineparser;
mod dirsum;

use std::fs;
use std::collections::HashMap;

use lineparser::Parser;
use dirsum::DirSum;

fn main() {
    let content = fs::read_to_string("input.txt").expect("file not found");
    let parser = Parser::new(&content);
    let parser_data = parser.parse();

    let dirsum = DirSum::new(&parser_data);
    // let answer = dirsum.get_size_of_lt_100000();
    // println!("the answer is {}", answer);

    let (dir, size) = dirsum.get_dir_to_delete(30000000, 70000000);

    // dbg!(&parser_data);
    // dbg!(&dirsum);

    println!("the answer is {} with {}", dir, size);
}
