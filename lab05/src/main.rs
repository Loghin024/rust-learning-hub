use std::{fs, io};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Student {
    name:String,
    phone:String,
    age:u32,
}

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let s = fs::read_to_string(file_path)?;
    Ok(s)
}

fn pb1()
{   
    let mut young = Student{name:String::from(""), phone:String::from(""), age:u32::MAX};
    let mut old = Student{name:String::from(""), phone:String::from(""), age:0};
    let file_content = read_file_content("resources/pb1.txt");
    let text = file_content.unwrap();
    for line in text.lines(){
        let mut it  = line.split(",");
        let mut name = String::from("");
        let mut phone = String::from("");
        let mut age;
        let mut field = 0;
        while let Some(v) = it.next() {
            if field == 0 {
                name.push_str(v);
            }
            else if field == 1 {
                phone.push_str(v);
            }   
            else if field == 2 {
                age = v.trim().parse().unwrap();

                if age > old.age {
                    old.age = age;
                    old.name.clear();
                    old.name.push_str(&name);
                    old.phone.clear();
                    old.phone.push_str(&phone);
                }
    
                if age < young.age {
                    // println!("{}, {}, {}", age, name, phone);
                    young.age = age;
                    young.name.clear();
                    young.name.push_str(&name);
                    young.phone.clear();
                    young.phone.push_str(&phone);
                }
            }
            field += 1;
        }
    }
    println!("The oldest person is: {:?}", old);
    println!("The youngest person is: {:?}", young);
}

fn new_canvas() -> [[char;50];10]{
    let a:[[char;50];10] = [[' '; 50];10];
    return a;
}

fn set_pixels(c: &mut[[char;50];10], commands: &[(u32,u32,u32)]) {
    for command in commands{
        c[command.0 as usize][command.1 as usize] = (command.2 as u8) as char;
    }
}

fn print(c: &[[char;50];10]){
    for i in 0..10{
        for j in 0..50{
            print!("{}", c[i][j]);
        }
        print!("\n");
    }
}

fn pb2()
{
    let mut canvas = new_canvas();
    let c = &mut canvas;

    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(c, &[(2, 16, 95), (5, 35, 92), (6, 3, 95), (2, 11, 95), (5, 3, 95)]);
    set_pixels(c, &[(2, 38, 95), (4, 9, 40), (3, 41, 124), (2, 37, 95), (2, 25, 124)]);
    set_pixels(c, &[(5, 27, 124), (2, 27, 124), (4, 0, 124), (3, 35, 47), (2, 18, 95)]);
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(c, &[(6, 30, 95), (5, 45, 95), (4, 31, 124), (4, 7, 124), (3, 43, 39)]);
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(c, &[(3, 44, 95), (3, 19, 92), (5, 23, 95), (3, 8, 47), (2, 10, 95)]);
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(c, &[(3, 10, 95), (4, 44, 95), (2, 9, 95), (0, 32, 95), (5, 2, 95)]);
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(c, &[(3, 46, 92), (5, 25, 44), (1, 43, 124), (5, 46, 47), (3, 15, 47)]);
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(c, &[(5, 44, 95), (2, 2, 95), (5, 10, 95), (5, 9, 95), (4, 43, 124)]);
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(c, &[(4, 40, 124), (3, 3, 95), (4, 4, 124), (6, 31, 47), (3, 4, 96)]);
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(c, &[(3, 12, 92), (2, 32, 95), (5, 37, 95), (5, 26, 95), (5, 39, 47)]);
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(c, &[(5, 36, 95), (7, 30, 95), (6, 4, 47), (4, 24, 95), (1, 32, 95)]);
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);

    print(&canvas);
}

fn pb3(){
    let mut young = Student{name:String::from(""), phone:String::from(""), age:u32::MAX};
    let mut old = Student{name:String::from(""), phone:String::from(""), age:0};
    let content = fs::read_to_string("resources/pb3.json").unwrap();
    for line in content.lines(){
        let s: Student = serde_json::from_str(&line).unwrap();
        
        if s.age > old.age {
            old = s.clone();
        }

        if s.age < young.age {
            young = s.clone();
        }

    }
    println!("The oldest person is: {:?}", old);
    println!("The youngest person is: {:?}", young);
}

fn main() {
    pb1();
    pb2();
    pb3()
}
