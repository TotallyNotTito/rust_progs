//! Command-Line Modular exponentiation tool
//!
//! Tito Reinhart 2023


use std::env;

// Main function setting up data structure and checking proper assertions
fn main() {
    let mut numbers = Vec::new();
    //convert args to nums in data-structure
    for arg in env::args().skip(1) {
        let s: String = arg.to_string();
        let n: u64 = parsenums(&s);
        //assert values are less than u64 max
        assert!(n < u64::max_value());
        numbers.push(n);
    }

    if numbers.len() != 3 {
        eprintln!("Usage: exponents 3 numbers...");
        error();
    }
    //assert numbers meet criteria
    assert!(numbers[2] > 0 && (numbers[0] + numbers[1] > 0));
    let base = numbers[0];
    let exp = numbers[1];
    let modulo = numbers[2];
    //call main function
    let result = modexp(base, exp, modulo);

    println!("The modulo exponent of {:?} is {}", numbers, result);
}
//Function that calculates modulo exponent
fn modexp(base: u64, exp: u64, modulo: u64) -> u64 {
    let mut base = u128::from(base) % u128::from(modulo);
    let mut exp = exp;
    let mut result: u128 = 1;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % u128::from(modulo);
        }
        exp /= 2;
        base = (base * base) % u128::from(modulo);
    }

    u64::try_from(result).unwrap()
}
//parse nums function that gets argument and converts to u64
fn parsenums(s: &str) -> u64 {
    let num: u64 = s.parse().unwrap_or_else(|_| error());
    num
}
//error message is parsenums goes wrong
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");
    std::process::exit(1);
}

//tests for test_modexp
#[test]
fn test_modexp() {
    let bigm = u64::max_value() - 58;
    assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
    assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
    assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
    assert_eq!(4, modexp(10, 9, 6));
    assert_eq!(34, modexp(450, 768, 517));
}

