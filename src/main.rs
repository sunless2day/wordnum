use std::{
    env,
    io::{self, Write},
    process::ExitCode,
};

use crate::{lexer::lex, parser::Parser, token::Num};

mod lexer;
mod parser;
mod token;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        if let Err(e) = repl() {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    } else if !run_line(&args.join(" ")) {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run_line(line: &str) -> bool {
    let tokens = match lex(line) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: {e}");
            return false;
        }
    };

    match Parser::new(tokens).parse() {
        Ok(n) => {
            println!("{}", group_thousands(n));
            true
        }
        Err(e) => {
            eprintln!("error: {e}");
            false
        }
    }
}

fn repl() -> io::Result<()> {
    let mut line = String::new();
    loop {
        print!(">>> ");
        io::stdout().flush()?;

        line.clear();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break Ok(()), // this is EOF
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::InvalidData => {
                eprintln!("failed to read input.\n{e}");
                continue;
            }
            Err(e) => return Err(e),
        }

        if line.trim().is_empty() {
            continue;
        }

        run_line(&line);
    }
}

fn group_thousands(n: Num) -> String {
    // gets rid of the negative sign
    let digits = n.unsigned_abs().to_string();
    let mut out = String::new();

    for (i, c) in digits.chars().enumerate() {
        // puts a comma wherever the number of digits left is a multiple of 3
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }

    if n < 0 { format!("-{out}") } else { out }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grouping_works() {
        let cases = [
            (0, "0"),
            (101, "101"),
            (3152, "3,152"),
            (100_000, "100,000"),
            (900_000_030_062, "900,000,030,062"),
            (1_123_123_053, "1,123,123,053"),
            (-42, "-42"),
            (-1_000, "-1,000"),
        ];

        for (n, expected) in cases {
            assert_eq!(group_thousands(n), expected, "formatting {n}");
        }
    }
}
