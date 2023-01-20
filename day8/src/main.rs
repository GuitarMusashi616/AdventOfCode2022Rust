mod forest;
use forest::Forest;
use std::fs;

fn main() {
    // problem_1();
    problem_2();
}

fn problem_1() {
    let input = fs::read_to_string("src/input.txt").expect("file not found");
    forest::get_dim(&input);

    let mut forest: Forest<99, 99> = input.as_str().into();

    forest.mark_visible();
    println!("{:?}", forest);

    let count = forest.count_visible();
    println!("num_visible: {}", count)
}

fn problem_2() {
    let input = fs::read_to_string("src/input.txt").expect("file not found");
    forest::get_dim(&input);

    let mut forest: Forest<99, 99> = input.as_str().into();

    let max_scenic_score = forest.find_max_scenic_score();
    println!("max_scenic_score: {}", max_scenic_score)
}

fn basic_example() {
    let mut forest = Forest::<5, 5>::new(
        [
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]
    );

    forest.mark_visible();
    println!("{:?}", forest);
    let count = forest.count_visible();
    println!("num_visible: {}", count);
    assert_eq!(count, 21);
}