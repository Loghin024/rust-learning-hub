use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self};

struct Cache {
    cache: RefCell<HashMap<u64, bool>>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            cache: RefCell::new(HashMap::new()),
        }
    }

    fn is_prime(&self, num: u64) -> bool {
        if num <= 1 {
            return false;
        }
        for i in 2..=(num as f64).sqrt() as u64 {
            if num % i == 0 {
                return false;
            }
        }
        true
    }

    fn get_or_insert(&self, num: u64) -> bool {
        let mut cache = self.cache.borrow_mut();

        if let Some(&result) = cache.get(&num) {
            result
        } else {
            let result = self.is_prime(num);
            cache.insert(num, result);
            result
        }
    }
}

fn main() -> io::Result<()>{
    let cache = Cache::new();

    loop {
        println!("Enter a number:");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
    
        match buffer.trim().parse::<u64>() {
            Ok(num) => {
                let result = cache.get_or_insert(num);
                println!("Is prime: {}", result);
            }
            Err(_) => {
                println!("Invalid input!\nEnter a valid number!");
            }
        }
    }
}
