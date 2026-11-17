use std::fmt::{self, Display, Formatter};

use crate::token::{Num, ScaleWord, Token};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEnd,
    ScaleOutOfOrder {
        previous: ScaleWord,
        found: ScaleWord,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::UnexpectedToken(token) => write!(f, "unexpected '{token}'"),
            Self::UnexpectedEnd => write!(f, "unexpected end of input"),
            Self::ScaleOutOfOrder { previous, found } => {
                write!(f, "'{found}' can't come after '{previous}'")
            }
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek_at(&self, offset: usize) -> Option<Token> {
        self.tokens.get(self.pos + offset).copied()
    }

    fn peek(&self) -> Option<Token> {
        self.peek_at(0)
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.peek();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }
    // good to know if the next token can initialize a chunk (like "fifteen million")
    fn starts_chunk(&self) -> bool {
        matches!(
            self.peek(),
            Some(Token::Unit(_) | Token::Teen(_) | Token::Ten(_))
        )
    }

    pub fn parse(mut self) -> Result<Num, ParseError> {
        let negative = if self.peek() == Some(Token::Minus) {
            self.next();
            true
        } else {
            false
        };

        let value = if self.peek() == Some(Token::Zero) {
            self.next();
            0
        } else {
            self.parse_number()?
        };

        // left-over input is an error ("twelve three", "zero eight", etc)
        if let Some(token) = self.next() {
            return Err(ParseError::UnexpectedToken(token));
        }

        Ok(if negative { -value } else { value })
    }

    fn parse_number(&mut self) -> Result<Num, ParseError> {
        let mut total = 0;
        let mut previous: Option<ScaleWord> = None;

        loop {
            let chunk = self.parse_below_thousand()?;

            // if there is no scale word, we just add whatever the chunk was to the total
            let Some(Token::Scale(scale)) = self.peek() else {
                total += chunk;
                break;
            };

            // a scale word must be smaller than the one before it
            if let Some(prev) = previous
                && scale >= prev
            {
                return Err(ParseError::ScaleOutOfOrder {
                    previous: prev,
                    found: scale,
                });
            }

            self.next();
            total += chunk * scale.to_num();
            previous = Some(scale);

            // "three thousand" ends here if the token afterwards can't start a chunk.
            // "five million eight" keeps going since the next token is able to start a chunk
            if !self.starts_chunk() {
                break;
            }
        }
        Ok(total)
    }

    fn parse_below_thousand(&mut self) -> Result<Num, ParseError> {
        let mut total = 0;
        let mut found_something = false;

        // "a" functions like "one" only when it is the first token (after an optional minus),
        // and only if the token after is a scale word (like "million", or "hundred").
        match (self.peek(), self.peek_at(1)) {
            (Some(Token::A), Some(Token::Hundred)) => {
                self.next();
                self.next();
                total = 100;
                found_something = true;
            }
            (Some(Token::A), Some(Token::Scale(_))) => {
                // `parse_number` will deal with it
                self.next();
                return Ok(1);
            }
            _ => {}
        }

        if let (Some(Token::Unit(n)), Some(Token::Hundred)) = (self.peek(), self.peek_at(1)) {
            // we skip twice since we already found two tokens ahead that match what we wanted
            self.next();
            self.next();
            total = n * 100;
            found_something = true;
        }

        if self.starts_chunk() {
            total += self.parse_below_hundred()?;
            found_something = true;
        }

        if !found_something {
            return match self.peek() {
                Some(token) => Err(ParseError::UnexpectedToken(token)),
                None => Err(ParseError::UnexpectedEnd),
            };
        }
        Ok(total)
    }

    fn parse_below_hundred(&mut self) -> Result<Num, ParseError> {
        let token = self.next().ok_or(ParseError::UnexpectedEnd)?;

        match token {
            Token::Unit(n) | Token::Teen(n) => Ok(n),
            Token::Ten(n) => {
                if let Some(Token::Unit(u)) = self.peek() {
                    self.next();
                    Ok(n + u)
                } else {
                    Ok(n)
                }
            }
            other => Err(ParseError::UnexpectedToken(other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    #[test]
    fn parsing_numbers_under_100_works_fine() {
        let cases = [
            ("thirty-five", 35),
            ("sixty seven", 67),
            ("eighty, nine", 89),
            ("twelve", 12),
            ("three", 3),
        ];

        for (input, result) in cases {
            let tokens = lex(input).unwrap();
            assert_eq!(
                Parser::new(tokens).parse_below_hundred().unwrap(),
                result,
                "parsing {input:?}"
            );
        }
    }

    #[test]
    fn same_thing_as_above_but_with_numbers_below_1000() {
        let cases = [
            ("five hundred and thirty eight", 538),
            ("nine hundred and ninety nine", 999),
            ("one hundred and one", 101),
            ("four hundred and fifteen", 415),
        ];

        for (input, result) in cases {
            let tokens = lex(input).unwrap();
            assert_eq!(
                Parser::new(tokens).parse_below_thousand().unwrap(),
                result,
                "parsing {input:?}"
            );
        }
    }

    fn parse(input: &str) -> Result<Num, ParseError> {
        Parser::new(lex(input).unwrap()).parse()
    }

    #[test]
    fn parsing_full_numbers() {
        let cases = [
            ("One hundred and one", 101),
            ("Three thousand one hundred and fifty-two", 3152),
            (
                "Nine hundred billion, thirty thousand, sixty-two",
                900_000_030_062,
            ),
            ("zero", 0),
            ("minus zero", 0),
            ("negative forty-two", -42),
            ("one hundred thousand", 100_000),
            ("one hundred and six thousand", 106_000),
            ("one million and five", 1_000_005),
            ("a hundred", 100),
            ("a million", 1_000_000),
            ("a hundred thousand", 100_000),
            ("minus a thousand", -1_000),
            ("a hundred and five", 105),
            (
                "nine hundred ninety-nine quadrillion,
                 nine hundred ninety-nine trillion,
                 nine hundred ninety-nine billion,
                 nine hundred ninety-nine million,
                 nine hundred ninety-nine thousand,
                 nine hundred and ninety-nine",
                999_999_999_999_999_999,
            ),
        ];

        for (input, result) in cases {
            assert_eq!(parse(input), Ok(result), "parsing {input:?}");
        }
    }

    #[test]
    fn rejects_invalid_numbers() {
        let cases = [
            "",
            "minus",
            "one three seven",
            "twenty twelve",
            "forty twelve",
            "twelve three",
            "fifty hundred",
            "hundred hundred",
            "three thousand hundred",
            "five thousand two million",
            "million million",
            "five thousand thousand",
            "zero five",
            "five zero",
            "five minus three",
            "a",
            "a twenty",
            "twenty a",
            "a a hundred",
            "two million a thousand",
        ];

        for input in cases {
            assert!(
                parse(input).is_err(),
                "{input:?} should fail, got {:?}",
                parse(input)
            );
        }
    }
}
