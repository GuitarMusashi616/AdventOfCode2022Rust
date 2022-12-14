use std::{num::ParseIntError, str::{FromStr, Lines}};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseCommandError {
    ParseIntFailed(String),
    InvalidInput(String),
}

impl From<ParseIntError> for ParseCommandError {
    fn from(error: ParseIntError) -> Self {
        ParseCommandError::ParseIntFailed(error.to_string())
    }
}

pub fn parse_command(command_str: &str) -> Result<[usize; 3], ParseCommandError> {
    // get word iter
    // grab the odd words
    // parse as number
    // put in output array
    let mut output = [0; 3];

    let mut index = 0;

    for (i, word) in command_str.split_ascii_whitespace().enumerate() {
        if i%2==1 {
            let num = word.parse()?;
            match output.get_mut(index) {
                Some(cell) => {
                    *cell = num;
                    index += 1;
                },
                None => return Err(ParseCommandError::InvalidInput("more than 3 numbers".to_string())),
            };
        }
    }

    if index != 3 {
        Err(ParseCommandError::InvalidInput("does not have exactly 3 numbers".to_string()))
    } else {
        Ok(output)
    }
}

pub fn parse_commands(content: & str) -> impl Iterator<Item=[usize; 3]> + '_ {
    CommandIter::new(content)
}

pub struct CommandIter<'a> {
    lines: Lines<'a>,
}

impl<'a> CommandIter<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            lines: content.lines(),
        }
    }
}

impl<'a> Iterator for CommandIter<'a> {
    type Item = [usize; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?.trim();
        parse_command(line).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "move 1 from 2 to 1";
        let res = parse_command(input).unwrap();
        let exp = [1, 2, 1];
        assert_eq!(res, exp);

        let input = "move 1 from 2";
        let res = parse_command(input);
        let exp = Err(ParseCommandError::InvalidInput("does not have exactly 3 numbers".to_string()));
        assert_eq!(res, exp);

        let input = "move from 2 to 3";
        let res = parse_command(input);
        let exp = Err(ParseCommandError::ParseIntFailed("invalid digit found in string".to_string()));
        assert_eq!(res, exp);

        let input = "move 1 from 2 to 3 and 4";
        let res = parse_command(input);
        let exp = Err(ParseCommandError::InvalidInput("more than 3 numbers".to_string()));
        assert_eq!(res, exp);
    }
}


