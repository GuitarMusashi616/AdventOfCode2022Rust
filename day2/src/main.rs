use std::{fs, collections::HashMap};


fn main() {
    let content = fs::read_to_string("input.txt").expect("file not found");
    let score = get_score(&content);
}

fn get_score(content: &str) -> i32 {
    let score_dict = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);
    let choice_dict = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let outcome_dict = HashMap::from([
        (("A", "X"), "C"),
        (("A", "Y"), "A"),
        (("A", "Z"), "B"),
        (("B", "X"), "A"),
        (("B", "Y"), "B"),
        (("B", "Z"), "C"),
        (("C", "X"), "B"),
        (("C", "Y"), "C"),
        (("C", "Z"), "A"),
    ]); 

    let mut total = 0;

    for mut split in content.lines().map(|x|x.trim().split(' ')) {
        if let (Some(first), Some(second)) = (split.next(), split.next()) {
            let pick = outcome_dict[&(first, second)];

            let choice = choice_dict[pick];
            let score = score_dict[second];

            let result = score + choice;
            println!("score {} {}: {} + {} = {}", first, second, score, choice, result);
            total += result;
        }
    }
    println!("total: {}", total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = "A Y\nB X\nC Z\n";
        let score = get_score(input);
        assert_eq!(score, 12);
    }
}

