use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

// just in case I want to change the type in the future
pub type Num = i64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Zero,
    // sometimes you want to say "a million" or "a hundred",
    // this is the token for that
    A,
    Unit(Num),
    Teen(Num),
    Ten(Num),
    Hundred,
    Scale(ScaleWord),
    Minus,
}

// insane macro hack as you can see
macro_rules! scale_words {
    ($($variant:ident => $name:literal, $value:expr;)*) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ScaleWord {
            $($variant,)*
        }

        impl ScaleWord {
            pub const ALL: &'static [ScaleWord] = &[$(Self::$variant,)*];

            pub fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }

            pub fn to_num(self) -> Num {
                match self {
                    $(Self::$variant => $value,)*
                }
            }

            pub fn from_word(word: &str) -> Option<Self> {
                Self::ALL.iter().copied().find(|s| s.name() == word)
            }
        }
    };
}

scale_words! {
    Thousand => "thousand", 1_000;
    Million => "million", 1_000_000;
    Billion => "billion", 1_000_000_000;
    Trillion => "trillion", 1_000_000_000_000;
    Quadrillion => "quadrillion", 1_000_000_000_000_000;
}

pub const UNITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

pub const TENS: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

impl PartialOrd for ScaleWord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_num().partial_cmp(&other.to_num())
    }
}

impl Display for ScaleWord {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "zero"),
            Self::A => write!(f, "a"),
            Self::Unit(n) => write!(f, "{}", UNITS[(*n - 1) as usize]),
            Self::Teen(n) => write!(f, "{}", TEENS[(*n - 10) as usize]),
            Self::Ten(n) => write!(f, "{}", TENS[(*n / 10 - 2) as usize]),
            Self::Hundred => write!(f, "hundred"),
            Self::Scale(scale) => write!(f, "{scale}"),
            Self::Minus => write!(f, "minus"),
        }
    }
}
