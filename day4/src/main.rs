mod pair_iter;
use std::fs;

use pair_iter::PairIter;

fn main() {
    let content = fs::read_to_string("input.txt").expect("file not found");
    let res: i32 = PairIter::new(&content).fold(0, |acc, (x, y)| {
        if any_overlap(x, y) {
            // dbg!((x, y));
            return acc + 1;
        }
        acc
    });
    println!("answer: {}", res);
}

pub fn does_pair_overlap((a, b): (i32, i32), (x, y): (i32, i32)) -> bool {
    a <= x && y <= b
}

pub fn any_overlap((a, b): (i32, i32), (x, y): (i32, i32)) -> bool {
    !(a < x && b < x || a > y && b > y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = "2-4,6-8
                     2-3,4-5
                     5-7,7-9
                     2-8,3-7
                     6-6,4-6
                     2-6,4-8";

        let res: i32 = PairIter::new(input).fold(0, |acc, (x, y)| {
            if does_pair_overlap(x, y) || does_pair_overlap(y, x) {
                // dbg!((x, y));
                return acc + 1;
            }
            acc
        });

        let exp = 2;
        assert_eq!(res, exp);
        
    }

    #[test]
    fn test_case2() {
        let input = "2-4,6-8
                     2-3,4-5
                     5-7,7-9
                     2-8,3-7
                     6-6,4-6
                     2-6,4-8";

        let res: i32 = PairIter::new(input).fold(0, |acc, (x, y)| {
            if any_overlap(x, y) {
                // dbg!((x, y));
                return acc + 1;
            }
            acc
        });

        let exp = 4;
        assert_eq!(res, exp);
        
    }

    #[test]
    fn test_does_pair_overlap() {
        let pair1 = (6, 6);
        let pair2 = (4, 6);

        let res1 = does_pair_overlap(pair1, pair2);
        assert!(!res1);

        let res2 = does_pair_overlap(pair2, pair1);
        assert!(res2);
    }

    #[test]
    fn test_does_pair_overlap_2() {
        let pair1 = (2, 4);
        let pair2 = (6, 8);

        let res1 = does_pair_overlap(pair1, pair2);
        assert!(!res1);

        let res2 = does_pair_overlap(pair2, pair1);
        assert!(!res2);
    }
}
