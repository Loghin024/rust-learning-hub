use std::fs;

// use anyhow::Result;
trait Command{
    fn get_name(&self)->&'static str;
    fn exec(&mut self, args: &[&str])->anyhow::Result<()>;
}

struct PingCommand{}
impl Command for PingCommand{
    fn get_name(&self)->&'static str {
        return "ping";
    }
    
    fn exec(&mut self, _: &[&str]) -> anyhow::Result<()> {
        println!("pong!");
        Ok(())
    }
}

struct CountCommand{}
impl Command for CountCommand{
    fn get_name(&self)->&'static str {
        return "count";
    }
    fn exec(&mut self, args: &[&str]) -> anyhow::Result<()> {
        println!("{}", args.len());
        Ok(())
    }
}

struct TimesCommand{
    count:u32
}
impl Command for TimesCommand{
    fn get_name(&self)->&'static str {
        return "times";
    }
    fn exec(&mut self, _: &[&str]) -> anyhow::Result<()> {
        self.count += 1;
        println!("{}", self.count);
        Ok(())
    }
}

struct ParityArgsCommand{}

impl Command for ParityArgsCommand{
    fn get_name(&self)->&'static str {
        return "parity_args";
    }
    fn exec(&mut self, args: &[&str]) -> anyhow::Result<()> {
        println!("{}", args.len()%2);
        Ok(())
    }
}

#[derive(Default)]
struct Terminal{
    commands:Vec<Box<dyn Command>>
}

impl Terminal{
    fn new()-> Terminal {
        Terminal::default()
    }
    fn register(&mut self, command:Box<dyn Command>){
        self.commands.push(command);
    }
    fn run(&mut self){
        match fs::read_to_string("resources/commands.txt") {
            Ok(file_content) => {
                for command_line in file_content.lines(){
                    let command: Vec<&str> = command_line.split_whitespace().collect();
                    if command.len() == 0{
                        continue;
                    }
                
                    for c in &mut self.commands{
                        if c.get_name().to_lowercase() == command[0].to_lowercase(){
                            match c.exec(&command[1..]) {
                                Ok(()) => println!("Command {} executed succesfully!", command[0]),
                                Err(err) => println!("Error at executing command {}\n Possible reason: {}", command[0], err) 
                            }
                        }
                        else if c.get_name().to_lowercase() == "stop"{
                            return;
                        }
                    }
                }
            }
            Err(err)=>println!("Error at opening commands.txt.\n Possible reason: {}", err)
        }

    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(ParityArgsCommand{}));
    
    terminal.run();
}