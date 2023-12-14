// encoder/src/main.rs
use std::fs;
use std::io;
use base64::encode;
use clap::Parser;

#[derive(Parser,Default,Debug)]
struct Arguments {
    input:Option<String>,
    output:Option<String>,
}

#[cfg(target_os = "windows")]
fn get_os_name()->&'static str {
"Windows"
}
#[cfg(target_os = "linux")]
fn get_os_name()->&'static str {
"Linux"
}

#[cfg(target_os = "macos")]
fn get_os_name()->&'static str {
"MacOS"
}


fn print_crate_info() {
    println!("encoder, version {}, built for:{}", env!("CARGO_PKG_VERSION"), get_os_name());
}

fn encode_and_output(input_bytes: &[u8]) {
    let encoded = encode(input_bytes);
    println!("{}", encoded);
}


fn main() {
    // Print crate information
    print_crate_info();

    // Parse command line arguments
    let args = Arguments::parse();

    match(args.input, args.output) {
        (Some(value1), Some(value2)) => {
            // Both options have values
            let input_file = &value1;
            let output_file = &value2;

            // Read bytes from the input file
            let input_bytes = match fs::read(input_file) {
                Ok(bytes) => bytes,
                Err(err) => {
                    eprintln!("Error reading input file: {}", err);
                    return;
                }
            };

            // Encode the input bytes and write to the output file
            encode_and_output(&input_bytes);

            // Write the base64 to the output file
            match fs::write(output_file, encode(&input_bytes)) {
                Ok(_) => println!("Base64 encoding successful. Output written to {}", output_file),
                Err(err) => eprintln!("Error writing to output file: {}", err),
            }
        }
        (Some(_), None) => {
            println!("Invalid number of arguments!");
        }
        (None, Some(_)) => {
            println!("Invalid number of arguments!");
        }
        (None, None) => {
            // If no input and output arguments are provided, read from stdin
            let mut input = String::new();
            if let Err(err) = io::stdin().read_line(&mut input) {
                eprintln!("Error reading from stdin: {}", err);
                return;
            }

            let input_bytes = input.trim().as_bytes();
            encode_and_output(input_bytes);
        }
    };    

}
