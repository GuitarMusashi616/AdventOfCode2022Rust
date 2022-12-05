// use std::env;
use std::{fs, collections::BinaryHeap};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file not found");
    // println!("content: {}", contents);
    let mut calories_per_elf = vec![0];
    let mut elf_num = 0;

    for (i, line) in contents.lines().map(|x|x.trim()).enumerate() {
        println!("line {}: {}", i, line);
        if line.is_empty() {
            println!("HERE");
            elf_num += 1;
            calories_per_elf.push(0);
            continue
        }

        let calories: i32 = line.parse().expect("could not find number in this string");
        calories_per_elf[elf_num] += calories;
    }


    let mut heap: BinaryHeap<i32> = calories_per_elf.into_iter().collect();
    let mut sum = 0;
    for i in 1..4 {
        let value = heap.pop().expect("can't pop from empty heap");
        println!("#{}: {:?}", i, value);
        sum += value;
    }
    println!("total: {}", sum);
    

    // let answer = calories_per_elf.iter().max();
    // dbg!(answer);
}
