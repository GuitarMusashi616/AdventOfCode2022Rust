use std::fs;

const A: usize = 97;

fn main() {
    // index based
    // grow / shrink
    // must be length of 4

    let input = fs::read_to_string("input.txt").expect("file not found");
    match first_marker(&input, 14) {
        Ok(res) => println!("the answer is {}", res),
        Err(err) => println!("didn't work because {:?}", err),
    }
}

#[derive(Debug)]
enum Problem {
    NonLowercaseAsciiChars,
    NoFirstMarker,
}

struct CharCount {
    counts: [u32; 26],
}

impl CharCount {
    fn new() -> Self {
        Self { counts: [0; 26] }
    }

    fn add(&mut self, chr: char) {
        let index = chr as usize - A;
        self.counts[index] += 1;
    }

    fn rem(&mut self, chr: char) {
        let index = chr as usize - A;
        self.counts[index] -= 1;
    }

    fn len(&self) -> usize {
        self.counts.len()
    }

    fn has_repeated_chars(&self) -> bool {
        self.counts.iter().find(|&&x| x >= 2).is_none()
    }
}

fn first_marker(input: &str, max: usize) -> Result<usize, Problem> {
    let mut chars: Vec<char> = input.chars().collect();

    let mut char_count = CharCount::new();

    assert!(char_count.len() >= max);
    for k in 0..max {
        let chr = chars[k];
        char_count.add(chr);
    }

    let mut i = 0;
    let mut j = max;

    loop {
        // if there are no repeated chars then break out of the loop
        if char_count.has_repeated_chars() {
            break;
        }

        // otherwise check the next char and remove the oldest char, update counts
        if j >= input.len() {
            return Err(Problem::NoFirstMarker);
        }

        char_count.rem(chars[i]);
        char_count.add(chars[j]);

        i += 1;
        j += 1;
    }

    Ok(j)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let max = 4;

        let str1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let str2 = "nppdvjthqldpwncqszvftbrmjlhg";
        let str3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let str4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        let res1 = first_marker(str1, max).unwrap();
        let res2 = first_marker(str2, max).unwrap();
        let res3 = first_marker(str3, max).unwrap();
        let res4 = first_marker(str4, max).unwrap();

        assert_eq!(res1, 5);
        assert_eq!(res2, 6);
        assert_eq!(res3, 10);
        assert_eq!(res4, 11);
    }

    #[test]
    fn test_case2() {
        let max = 14;

        let str1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let str2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let str3 = "nppdvjthqldpwncqszvftbrmjlhg";
        let str4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let str5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        let res1 = first_marker(str1, max).unwrap();
        let res2 = first_marker(str2, max).unwrap();
        let res3 = first_marker(str3, max).unwrap();
        let res4 = first_marker(str4, max).unwrap();
        let res5 = first_marker(str5, max).unwrap();

        assert_eq!(res1, 19);
        assert_eq!(res2, 23);
        assert_eq!(res3, 23);
        assert_eq!(res4, 29);
        assert_eq!(res5, 26);
    }

    #[test]
    fn test_char_counter() {
        let mut char_counter = CharCount::new();
        assert_eq!(char_counter.counts[2], 0);
        char_counter.add('c');
        char_counter.add('c');
        assert_eq!(char_counter.counts[2], 2);
        char_counter.add('a');
        assert_eq!(char_counter.counts[0], 1);
        char_counter.rem('c');
        assert_eq!(char_counter.counts[2], 1);
    }
}
