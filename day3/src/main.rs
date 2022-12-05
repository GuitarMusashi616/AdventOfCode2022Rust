use std::{collections::HashSet, fs, str::Lines};

const LOWER: i32= 96;

fn main() {
    let content = fs::read_to_string("input.txt").expect("file not found");
    let chrs = get_all_common_chars(&content);
    let total = process_chars(chrs);
    dbg!(total);
}

pub fn get_common_chars(items: &str) -> char {
    assert!(items.len()%2==0);
    let n = items.len() / 2;
    let rucksack1: HashSet<char> = items.chars().take(n).collect();
    let rucksack2: HashSet<char> = items.chars().skip(n).collect();

    rucksack1.into_iter().find(|c| rucksack2.contains(c)).unwrap_or('+')
}

pub fn get_priority(chr: char) -> i32 {
    let mut bonus = 0;
    if chr.is_uppercase() {
        bonus = 26;
    }
    chr.to_ascii_lowercase() as i32 - LOWER + bonus
}

pub fn get_common_char_from_3(iter: &mut Lines) -> Option<char> {
    // given an iterator will save the next 3 lines, see what is common about all 3, and return
    // that character
    let mut lines = Vec::new();
    for _ in 0..3 {
        let line = iter.next()?.trim();
        lines.push(line);
    }

    let rucksack1: HashSet<char> = lines[0].chars().collect();
    let rucksack2: HashSet<char> = lines[1].chars().collect();

    lines[2].chars().find(|x| rucksack1.contains(x) && rucksack2.contains(x))
}

pub fn get_all_common_chars(content: &str) -> Vec<char> {
    let mut iter = content.lines();
    let mut res = Vec::new();

    while let Some(chr) = get_common_char_from_3(&mut iter) {
        res.push(chr);
    }

    res
}

pub fn process_chars(chrs: Vec<char>) -> i32 {
    let mut total = 0;
    for chr in chrs {
        total += get_priority(chr);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let content = "vJrwpWtwJgWrhcsFMMfFFhFp
                       jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                       PmmdzqPrVvPwwTWBwg
                       wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                       ttgJtRGJQctTZtZT
                       CrZsJsPPZsGzwwsLwLmpwMDw";

        let mut res = vec![];
        let mut res2 = vec![];
        let exp = vec!['p', 'L', 'P', 'v', 't', 's'];
        let exp2 = vec![16, 38, 42, 22, 20, 19];


        let mut total = 0;
        for line in content.split('\n').map(|x|x.trim()) {
            let chr = get_common_chars(line);
            let prio = get_priority(chr); 
            res.push(chr);
            res2.push(prio);
            total += prio;
            println!("{}: {}", line, chr);
        }
        assert_eq!(res, exp);
        assert_eq!(res2, exp2);
        assert_eq!(total, 157);
    }

    #[test]
    fn test_case_2() {
        let content =  "vJrwpWtwJgWrhcsFMMfFFhFp
                        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                        PmmdzqPrVvPwwTWBwg
                        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                        ttgJtRGJQctTZtZT
                        CrZsJsPPZsGzwwsLwLmpwMDw";

        let mut iter = content.lines();
        let mut res = Vec::new();
        let exp = vec!['r', 'Z'];

        while let Some(chr) = get_common_char_from_3(&mut iter) {
            res.push(chr);
        }

        assert_eq!(res, exp);

        
        let total = process_chars(res);
        assert_eq!(total, 70);
    }
}
