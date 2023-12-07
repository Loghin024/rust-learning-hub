use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn read_file(file: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file)?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn solve(content:String)
{
    let mut map = HashMap::new();
    let mut word = content.split(|c: char| !c.is_alphanumeric());
    while let Some(w) = word.next() {
        if w != "" {
            *map.entry(w.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }

    //collect data into a vector
    let mut word_count_vec: Vec<_> = map.into_iter().collect();

    //sort data
    word_count_vec.sort_by(|a, b| b.1.cmp(&a.1));

    //print formatted data
    for (word, count) in word_count_vec {
        println!("{:<10} => {}", word, count);
    }
}

fn main() {
    let path = "resources/input.txt";
    match read_file(path) {
        Ok(content)=>solve(content),
        Err(err)=>println!("Unable to read '{}': {}", path, err)
    }
}
