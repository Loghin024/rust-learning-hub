fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn problem1() {
    for i in 0..100 {
        if is_prime(i) {
            println!("{}", i)
        }
    }
}

fn gcd(mut x: u32, mut y: u32) -> u32 {
    while y != 0 {
        let r = y;
        y = x % y;
        x = r;
    }
    x
}

fn are_coprime(x: u32, y: u32) -> bool {
    gcd(x, y) == 1
}

fn problem2() {
    for i in 0..100 {
        for j in 0..100 {
            if are_coprime(i, j) {
                println!("{},{}", i, j)
            }
        }
    }
}

fn problem3() {
    for bottle in (1..100).rev() {
        println!("{} bottles of beer on the wall,", bottle);
        println!("{} bottles of beer.", bottle);
        println!("Take one down, pass it around,");
        if bottle == 1 {
            println!("No bottles of beer on the wall,");
            println!("");
        } else {
            println!("{} bottles of beer on the wall,", bottle - 1);
            println!("");
        }
    }
}

fn main() {
    problem1();
    problem2();
    problem3();

    //Made with ChatGPT 3.0
    //(Joke...)
}
