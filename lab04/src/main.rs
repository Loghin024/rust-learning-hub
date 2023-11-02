use std::{fs, io};

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let s = fs::read_to_string(file_path)?;
    Ok(s)
}

fn pb1() {
    let mut longest_line_ch = String::from("");
    let mut l_ch: u32 = 0;
    let mut l_bytes: u32 = 0;
    let mut longest_line_bytes = String::from("");
    let file_content = read_file_content("resources/file.txt");

    for line in file_content.unwrap().lines() {
        if line.len() as u32 > l_bytes {
            l_bytes = line.len() as u32;
            longest_line_bytes.clear();
            longest_line_bytes.push_str(line);
        }
        let mut character_counter = 0;
        for _ in line.chars() {
            character_counter += 1;
        }

        if character_counter > l_ch {
            l_ch = character_counter;
            longest_line_ch.clear();
            longest_line_ch.push_str(line);
        }
    }
    println!("Longest line(bytes): {}", longest_line_bytes);
    println!("Longest line(ch): {}", longest_line_ch);
}

fn pb2() {
    let input = String::from("##heLlo&&&");
    let mut encrypted_string = String::from("");

    for ch in input.chars() {
        if !ch.is_ascii() {
            panic!("this character is not ascii!");
        }
        let mut offset: u8;
        if ch >= 'a' && ch <= 'z' {
            offset = (ch as u8) - 'a' as u8;
            offset = (offset + 13) % 26;
            encrypted_string.push((offset + 'a' as u8) as char);
        }

        if ch >= 'A' && ch <= 'Z' {
            offset = (ch as u8) - 'A' as u8;
            offset = (offset + 13) % 26;
            encrypted_string.push((offset + 'A' as u8) as char);
        }
    }
    println!("{}", encrypted_string);
}

fn pb3() {
    let file_content = read_file_content("resources/p3.txt");
    let mut final_text = String::from("");
    let text = file_content.unwrap();
    let mut it = text.split(" ");
    while let Some(v) = it.next() {
        if v.eq("pt") {
            final_text.push_str("pentru")
        } else if v.eq("ptr") {
            final_text.push_str("pentru")
        } else if v.eq("dl") {
            final_text.push_str("domnul")
        } else if v.eq("dna") {
            final_text.push_str("doamna")
        } else {
            final_text.push_str(v);
        }
        final_text.push(' ');
    }
    println!("{}", final_text);
}

fn pb4() {
    let host_file = read_file_content(r"resources/p4.txt");
    let host_file_text = host_file.unwrap();

    for line in host_file_text.lines() {
        let trimmed_line = line.trim();

        if !trimmed_line.is_empty() && !trimmed_line.starts_with("#") {
            let mut columns = trimmed_line.split_whitespace();

            if let Some(first) = columns.next() {
                if let Some(second) = columns.next() {
                    println!("{} => {}", first, second);
                }
            }
        }
    }
}

fn main() {
    pb1();
    pb2();
    pb3();
    pb4();
}
