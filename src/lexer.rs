use std::fmt::{self, Display, Formatter};

use crate::token::{Num, ScaleWord, TEENS, TENS, Token, UNITS};

#[derive(Debug)]
pub enum LexError {
    UnknownWord(String),
}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::UnknownWord(word) => write!(f, "unknown word '{word}'"),
        }
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();

    let lowered = input.to_lowercase();

    let words = lowered
        .split(|c: char| c.is_whitespace() || c == '-' || c == ',')
        .filter(|w| !w.is_empty());

    for word in words {
        if word == "and" {
            continue;
        }
        let token = word_to_token(word).ok_or_else(|| LexError::UnknownWord(word.to_string()))?;
        tokens.push(token);
    }

    Ok(tokens)
}

fn word_to_token(word: &str) -> Option<Token> {
    if let Some(i) = UNITS.iter().position(|&w| w == word) {
        return Some(Token::Unit(1 + i as Num));
    }

    if let Some(i) = TEENS.iter().position(|&w| w == word) {
        return Some(Token::Teen(10 + i as Num));
    }

    if let Some(i) = TENS.iter().position(|&w| w == word) {
        return Some(Token::Ten((2 + i as Num) * 10));
    }

    if let Some(scale) = ScaleWord::from_word(word) {
        return Some(Token::Scale(scale));
    }

    match word {
        "zero" => Some(Token::Zero),
        "a" => Some(Token::A),
        "hundred" => Some(Token::Hundred),
        "minus" | "negative" => Some(Token::Minus),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_compounds() {
        assert_eq!(
            lex("sixty-seven").unwrap(),
            vec![Token::Ten(60), Token::Unit(7)]
        );
    }

    #[test]
    fn lexes_teen_numbers() {
        let teens = [
            ("eleven", 11),
            ("twelve", 12),
            ("thirteen", 13),
            ("fifteen", 15),
            ("sixteen", 16),
            ("seventeen", 17),
            ("eighteen", 18),
            ("nineteen", 19),
        ];

        for (word, value) in teens {
            assert_eq!(
                lex(word).unwrap(),
                vec![Token::Teen(value)],
                "lexing {word:?}"
            );
        }
    }
}
