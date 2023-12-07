use std::fmt;
use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
struct RequestResult {
    results:Vec<DndSpell>,
}

#[derive(Deserialize)]
struct DndSpell{
    index: String,
    name: String,
    url: String,
}
#[derive(Deserialize)]
struct DndSpellV2{
    name: String,
    level: i32,
    desc: Vec<String>,
}

impl fmt::Display for DndSpell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Index: {}\nName: {}\nURL: {}\n",
            self.index, self.name, self.url
        )
    }
}

impl fmt::Display for DndSpellV2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nLevel: {}\nDescription: {}\n",
            self.name, self.level, self.desc[0]
        )
    }
}

fn show_spell_info(path_to_spell:&str)-> Result<(), ureq::Error>{
    let body: String = ureq::get(&path_to_spell)
        .call()?
        .into_string()?;
    let result: Result<DndSpellV2, serde_json::Error> = serde_json::from_str(&body);

    match result {
        Ok(parsed_struct) => {
            println!("{}", parsed_struct);
        }
        Err(err) => {
            println!("Error parsing JSON: {}", err);
        }
    }

    Ok(())
}

fn solve(input:String)-> Result<(), ureq::Error> {
    let body: String = ureq::get("https://www.dnd5eapi.co/api/spells")
        .call()?
        .into_string()?;
    let result: Result<RequestResult, serde_json::Error> = serde_json::from_str(&body);
    // Handle the result
    match result {
        Ok(parsed_struct) => {
            for spell in parsed_struct.results{
                if !spell.name.to_ascii_lowercase().trim().contains(input.to_ascii_lowercase().trim()){
                    continue;
                }
                let path_to_spell:String= String::from("https://www.dnd5eapi.co")+ spell.url.as_str();
                match show_spell_info(path_to_spell.as_str()) {
                    // Ok(spell_info)=>println!("{}", spell_info),
                    Ok(_)=>println!(""),
                    Err(err)=>println!("Request failed!\nPossible reason:{}", err)
                }            
            }
        }
        Err(err) => {
            println!("Error parsing JSON: {}", err);
        }
    }
    Ok(())
}

fn main()-> io::Result<()> {
    println!("Enter spell name: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    match solve(buffer) {
        Ok(_)=>println!("Done!"),
        Err(err)=>println!("Error:{}", err)
    }

    Ok(())
}
