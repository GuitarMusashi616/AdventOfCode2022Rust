mod crates;
mod parse_cmd;

use parse_cmd::{parse_command, parse_commands, ParseCommandError};
use crates::Crates;
use std::{collections::VecDeque, fs};

fn main() {
    let content = fs::read_to_string("input.txt").expect("file not found");

    let mut crates = Crates::from([
        "WBDNCFJ",
        "PZVQLST",
        "PZBGJT",
        "DTLJZBHC",
        "GVBJS",
        "PSQ",
        "BVDFLMPN",
        "PSMFBDLR",
        "VDTR",
    ]);

    println!("{}", crates);
    crates.execute_ordered(&content).expect("didn't work");
    println!("{}", crates);
}

pub fn execute(content: &str) {
    // let content = fs::read_to_string("input.txt").expect("file not found");
    let mut crates = crates![
        ['Z', 'N'],
        ['M','C','D'],
        ['P']
    ];

    dbg!(crates);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {

        let mut crates = crates![
            ['Z', 'N'],
            ['M','C','D'],
            ['P']
        ];

        let content = "move 1 from 2 to 1
                       move 3 from 1 to 3
                       move 2 from 2 to 1
                       move 1 from 1 to 2";

        println!("{}", crates);
        let res = crates.execute(content).expect("didn't work");
        println!("{}", crates);
        assert_eq!(&res, "CMZ");
    }

    #[test]
    fn test_case_2() {

        let mut crates = crates![
            ['Z', 'N'],
            ['M','C','D'],
            ['P']
        ];

        let content = "move 1 from 2 to 1
                       move 3 from 1 to 3
                       move 2 from 2 to 1
                       move 1 from 1 to 2";

        println!("{}", crates);
        let res = crates.execute_ordered(content).expect("didn't work");
        println!("{}", crates);
        assert_eq!(&res, "MCD");
    }

}


