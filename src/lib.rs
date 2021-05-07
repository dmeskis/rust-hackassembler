use std::error::Error;
use std::fs;
use std::process;
use std::str::Lines;

pub struct Parser {
    pub filename: String,
    pub contents: String,
    // pub lines: Lines,
}

impl Parser {
    pub fn new(filename: String) -> Parser {
        let contents =
            fs::read_to_string(&filename).expect("Something went wrong reading the file.");
        let lines = contents.lines();
        Parser {
            filename,
            contents,
            // lines,
        }
    }

    // pub fn has_more_commands() -> Boolean {}

    // pub fn contents(&self) -> String {
    //     let contents =
    //         fs::read_to_string(&self.filename).expect("Something went wrong reading the file.");

    //     contents
    // }
}

// struct Command {
//     line: String,
// }

// impl Command {
//     fn new(line: String) -> Command {
//         Command { line }
//     }

//     // fn type {

//     // }
//     // fn symbol {

//     // }

//     // fn dest {

//     // }

//     // fn comp {

//     // }

//     // fn jump {

//     // }
// }

pub fn parse_args(args: &[String]) -> Result<String, &str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    let filename = args[1].clone();

    Ok(filename)
}

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let filename = parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let parser = Parser::new(filename);

    println!("With text:\n{}", parser.contents);

    Ok(())
}
