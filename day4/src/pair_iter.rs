use std::str::Lines;

pub struct PairIter<'a>  {
    lines: Lines<'a>,
}

impl<'a> PairIter<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            lines: content.lines(),
        }
    }

    pub fn split_half(line: &'a str, chr: char) -> Option<(&'a str, &'a str)> {
        let mut negsplit = line.split(chr);
        let a = negsplit.next()?;
        let b = negsplit.next()?;
        Some((a, b))
    }

    pub fn parse_pair(a: &str, b: &str) -> Option<((i32, i32), (i32, i32))> {
        let (x1, y1) = PairIter::split_half(a, '-')?;
        let (x2, y2) = PairIter::split_half(b, '-')?;

        let low1 = x1.parse().ok()?;
        let high1 = y1.parse().ok()?;
        let low2 = x2.parse().ok()?;
        let high2 = y2.parse().ok()?;
        Some(((low1, high1), (low2, high2)))
    }
}


impl<'a> Iterator for PairIter<'a> {
    type Item = ((i32, i32), (i32, i32));

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.lines.next() {
            let (a, b) = PairIter::split_half(line.trim(), ',')?;
            let (fig1, fig2) = PairIter::parse_pair(a, b)?;
            // println!("a = {:?}, b = {:?}", fig1, fig2);
            Some((fig1, fig2))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_pairs() {
        let input = "2-4,6-8
                     2-3,4-5
                     5-7,7-9
                     2-8,3-7
                     6-6,4-6
                     2-6,4-8";

        let pair_iter = PairIter::new(input);
        let res: Vec<_> = pair_iter.collect();
        let exp = vec![((2, 4), (6, 8)), ((2, 3), (4, 5)), ((5, 7), (7, 9)), ((2, 8), (3, 7)), ((6, 6), (4, 6)), ((2, 6), (4, 8))];
        assert_eq!(res, exp);
    }
}
