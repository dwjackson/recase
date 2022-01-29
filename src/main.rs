use std::env;
use std::io;
use recase::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("USAGE: recase [INPUT_TYPE] [OUTPUT_TYPE]");
        println!("Input is read from stdin.");
        println!("The types are:");
        println!("\tcamel - camelCase");
        println!("\tsnake - snake_case");
        println!("\tkebeb - kebab-case");
    } else {
        let in_case_type = match args[1].parse::<CaseType>() {
            Ok(case_type) => case_type,
            Err(_) => panic!("Invalid input case type: {}", &args[1]),
        };
        let out_case_type = match args[2].parse::<CaseType>() {
            Ok(case_type) => case_type,
            Err(_) => panic!("Invalid output case type: {}", &args[1]),
        };
        if args.len() == 4 {
                let output = convert(&args[3], in_case_type, out_case_type);
                println!("{}", output);
        } else {
            convert_from_stdin(in_case_type, out_case_type);
        } 
    }
}

fn convert_from_stdin(in_case_type: CaseType, out_case_type: CaseType) {
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        match stdin.read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let output = convert(&buffer, in_case_type, out_case_type);
                println!("{}", output.trim());
            },
            Err(_) => {
                break;
            }
        }
    }
}
