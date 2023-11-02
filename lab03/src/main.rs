fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    if n % 2 == 0 && n != 2 {
        return false;
    }

    for i in (3..=n).step_by(2).take_while(|i| i * i <= n) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

fn next_prime(x: u16) -> Option<u16> {
    let mut next_num: u32 = x as u32 + 1;
    let maxu16 = std::u16::MAX as u32;
    while next_num <= maxu16 {
        if is_prime(next_num) {
            break;
        }
        next_num += 1;
    }

    if next_num < maxu16 {
        return Some(next_num as u16);
    } else {
        return None;
    }
}

fn pb1() {
    let mut start = 1;
    while let Some(i) = next_prime(start) {
        println!("Urmatorul numar prim este: {}", i);
        start = i;
    }

    println!(
        "Nu s-a gasit un numar prim mai mare decat {} care poate fi reprezentat pe u16!",
        start
    );
}

fn check_add(a: u32, b: u32) -> u32 {
    let sum: u64 = a as u64 + b as u64;

    if sum > std::u32::MAX as u64 {
        panic!("Suma depaseste u32");
    }

    return sum as u32;
}
fn check_multiplication(a: u32, b: u32) -> u32 {
    let product: u64 = a as u64 * b as u64;
    if product > std::u32::MAX as u64 {
        panic!("Produsul celor doua numere depaseste u32");
    }
    return product as u32;
}

fn pb2() {
    print!("{}\n", check_add(5, 6));
    // print!("{}", check_add(std::u32::MAX, 1));
    print!("{}\n", check_multiplication(5, 6));
    // print!("{}", check_multiplication(std::u32::MAX, 2));
}

use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Overflow")]
    Overflow,
}

fn check_add_modified(a: u32, b: u32) -> Result<u32, MyError> {
    let sum: u64 = a as u64 + b as u64;
    if sum < std::u32::MAX as u64 {
        return Ok(sum as u32);
    } else {
        return Err(MyError::Overflow);
    }
}
fn check_multiplication_modified(a: u32, b: u32) -> Result<u32, MyError> {
    let product: u64 = a as u64 * b as u64;
    if product < std::u32::MAX as u64 {
        return Ok(product as u32);
    } else {
        return Err(MyError::Overflow);
    }
}

fn pb3() {
    let sum1 = check_add_modified(1, 2);
    match sum1 {
        Ok(number) => println!("{number}"),
        Err(e) => println!("Eroare: {e}"),
    }

    let product = check_multiplication_modified(2, 3);
    match product {
        Ok(num) => println!("{num}"),
        Err(e) => println!("Eroare: {e}"),
    }

    let sum_overflow = check_add_modified(std::u32::MAX, 1);
    match sum_overflow {
        Ok(num) => println!("{num}"),
        Err(e) => println!("Eroare: {e}"),
    }
    let product_overflow = check_multiplication_modified(std::u32::MAX, 2);
    match product_overflow {
        Ok(num) => println!("{num}"),
        Err(e) => println!("ERoare: {e}"),
    }
}

//problem 4
#[derive(Error, Debug)]
enum CustomError {
    #[error("Character is not ASCII")]
    NotAscii,
    #[error("Character is not a digit")]
    NotDigit,
    #[error("Character is not a base16 digit")]
    NotBase16Digit,
    #[error("Character is not a letter")]
    NotLetter,
    #[error("Character is not printable")]
    NotPrintable,
}

fn to_uppercase(character: char) -> Result<char, CustomError> {
    if character >= 'A' && character <= 'Z' {
        return Ok(character);
    } else {
        if character >= 'a' && character <= 'z' {
            // print!("lok {}\n", (character as u8 - b'A' + b'a') as char);
            return Ok(character.to_ascii_uppercase());
        } else {
            return Err(CustomError::NotLetter);
        }
    }
}

fn to_lowercase(character: char) -> Result<char, CustomError> {
    if character >= 'a' && character <= 'z' {
        return Ok(character);
    } else {
        if character >= 'A' && character <= 'Z' {
            return Ok(character.to_ascii_lowercase());
        } else {
            return Err(CustomError::NotLetter);
        }
    }
}

fn print_char(ch: char) -> Result<(), CustomError> {
    if (ch >= '0' && ch <= '9')
        || (ch >= 'a' && ch <= 'z')
        || (ch >= 'A' && ch <= 'Z')
        || ch == '!'
        || ch == '\"'
        || ch == '#'
        || ch == '$'
        || ch == '%'
        || ch == '&'
        || ch == '\''
        || ch == '('
        || ch == ')'
        || ch == '*'
        || ch == '+'
        || ch == ','
        || ch == '-'
        || ch == '.'
        || ch == '/'
        || ch == ':'
        || ch == ';'
        || ch == '<'
        || ch == '='
        || ch == '>'
        || ch == '?'
        || ch == '@'
        || ch == '['
        || ch == '\\'
        || ch == ']'
        || ch == '^'
        || ch == '`'
        || ch == '{'
        || ch == '|'
        || ch == '}'
    {
        Ok(())
    } else {
        Err(CustomError::NotPrintable)
    }
}

fn char_to_number(ch: char) -> Result<u32, CustomError> {
    if ch.is_ascii() && ch.is_digit(10) {
        Ok(ch.to_digit(10).unwrap())
    } else {
        if ch.is_ascii() {
            Err(CustomError::NotDigit)
        } else {
            Err(CustomError::NotAscii)
        }
    }
}

fn char_to_number_hex(ch: char) -> Result<u32, CustomError> {
    if ch.is_ascii() && ch.is_digit(16) {
        Ok(ch.to_digit(16).unwrap())
    } else {
        if ch.is_ascii() {
            Err(CustomError::NotBase16Digit)
        } else {
            Err(CustomError::NotAscii)
        }
    }
}

fn print_error(error: CustomError) {
    eprintln!("Error: {}", error);
}

fn pb4() {
    let mut input_char = 'a';

    match to_uppercase(input_char) {
        Ok(upper) => println!("Uppercase: {}", upper),
        Err(error) => eprintln!("Error: {}", error),
    }

    input_char = '$';
    match to_uppercase(input_char) {
        Ok(upper) => println!("Uppercase: {}", upper),
        Err(error) => eprintln!("Error: {}", error),
    }

    input_char = 'Z';
    match to_lowercase(input_char) {
        Ok(lower) => println!("Lowercase: {}", lower),
        Err(error) => eprintln!("Error: {}", error),
    }

    input_char = '*';
    match to_lowercase(input_char) {
        Ok(lower) => println!("Lowercase: {}", lower),
        Err(error) => eprintln!("Error: {}", error),
    }

    let printable_char = '!';
    match print_char(printable_char) {
        Ok(_) => println!("Printable character"),
        Err(error) => eprintln!("Error: {}", error),
    }

    let printable_char = '\n';
    match print_char(printable_char) {
        Ok(_) => println!("Printable character"),
        Err(error) => eprintln!("Error: {}", error),
    }

    let digit_char = '5';
    match char_to_number(digit_char) {
        Ok(num) => println!("Number: {}", num),
        Err(error) => eprintln!("Error: {}", error),
    }

    let hex_digit_char = 'C';
    match char_to_number_hex(hex_digit_char) {
        Ok(num) => println!("Hex Number: {}", num),
        Err(error) => eprintln!("Error: {}", error),
    }

    print_error(CustomError::NotAscii);
}

fn check_word(word: &str, swears: &[&str]) -> Result<(), ()> {
    if swears.contains(&word) {
        Err(())
    } else {
        Ok(())
    }
}

fn mini_app_swear_words_recognizer() {
    let word = "imi place rust";
    let swears = ["nu imi place rust", " ****  rust", "hmmm"];

    match check_word(&word, &swears) {
        Ok(_) => println!("The word is not a bad word."),
        Err(_) => println!("The word is a bad word."),
    }
}

fn main() {
    pb1();
    pb2();
    pb3();
    pb4();
    mini_app_swear_words_recognizer()
}
