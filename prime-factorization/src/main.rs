extern crate primal;

use primal::Sieve;
use std::io::{self, Write};
use std::fmt::Display;
use std::process;

fn prime_factors(n: usize) -> String {
    let sieve = Sieve::new(10000);
    match sieve.factor(n) {
        Ok(factors) => {
            factors.into_iter().fold(String::new(), |acc, (term, exponent)| {
                match (acc.is_empty(), exponent) {
                    (true, 1) => acc + &format!("{}", term),
                    (true, _) => acc + &format!("{}^{}", term, exponent),
                    (false, 1) => acc + &format!(" * {}", term),
                    (false, _) => acc + &format!(" * {}^{}", term, exponent),
                }
            })
        },
        Err(_) => format!("No prime factorization!"),
    }
}

fn main() {
    let n: usize = grab_input("Give me an integer greater than 0")
        .unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)))
        .trim()
        .parse()
        .unwrap_or_else(|e| exit_err(&e, 2));

    println!("Prime factorization: {}", prime_factors(n));
}

fn grab_input(msg: &str) -> io::Result<String> {
    let mut buf = String::new();
    print!("{}: ", msg);
    try!(io::stdout().flush());

    try!(io::stdin().read_line(&mut buf));
    Ok(buf)
}

fn exit_err<T: Display>(msg: T, code: i32) -> ! {
    let _ = writeln!(&mut io::stderr(), "Error: {}", msg);
    process::exit(code)
}
