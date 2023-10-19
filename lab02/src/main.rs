fn add_chars_n(mut s: String, c: char, n: u32) -> String {
    for _ in 0..n {
        s.push(c);
    }
    return s;
}

fn add_chars_n_modified(s: &mut String, c: char, n: u32) {
    for _ in 0..n {
        s.push(c);
    }
}

fn pb1() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}

fn pb2() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n_modified(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}

fn add_space(s: &mut String, n: u32) {
    for _ in 0..n {
        s.push(' ');
    }
}

fn add_str(s: &mut String, _s: &str) {
    s.push_str(_s)
}

fn add_integer(s: &mut String, mut n: u64) {
    //converting i32 into a string
    let mut s_num_rev = String::from("");
    while n != 0 {
        let digit: u8 = (n % 10) as u8;
        let c = (digit + b'0') as char;
        s_num_rev.push(c);
        n /= 10;
    }
    let mut counter = 0;
    for d in s_num_rev.chars().rev() {
        if counter % 3 == 0 && counter != 0 {
            s.push('_');
        }
        counter += 1;
        s.push(d);
    }
}

fn add_float(s: &mut String, input: f64, precision: u32) {
    let mut n = input;

    // Handle negative numbers
    if n < 0.0 {
        s.push('-');
        n = -n;
    }

    // Integer part
    let integer_part = n as u64;
    add_integer(s, integer_part);

    // Decimal point
    s.push('.');

    // Fractional part
    let fractional_part = n - n.floor();
    let mut remaining_fraction = fractional_part;
    for _ in 0..precision {
        remaining_fraction *= 10.0;
        let digit = (remaining_fraction as u32) % 10;
        s.push(char::from(digit as u8 + b'0'));
    }
}

fn pb3() {
    let mut s: String = String::from("");
    add_space(&mut s, 40);
    add_str(&mut s, "I");
    add_space(&mut s, 2);
    add_str(&mut s, "💚");
    add_space(&mut s, 40);
    add_str(&mut s, "\n");
    add_space(&mut s, 40);
    add_str(&mut s, "RUST");
    add_space(&mut s, 40);
    add_str(&mut s, "\n");
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 5);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "lastest");
    add_space(&mut s, 9);
    add_str(&mut s, "is");
    add_space(&mut s, 7);
    add_str(&mut s, "\n");
    add_space(&mut s, 5);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 7);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 9);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038, 3);
    add_str(&mut s, ".");

    print!("{}", s);
}

fn main() {
    pb1();
    println!("\n");
    pb2();
    println!("\n");
    pb3();
}
