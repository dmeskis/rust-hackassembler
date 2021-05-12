use regex::Regex;
use std::error::Error;
use std::fs;
use std::process;
// use std::str::Lines;

// stack variables as parameters are copied and put on the stack because they are so cheap
// heap variables are not, which is why compilation errors will be raised
//
// Allocated memory needs to have one, and only one owner (exceptions but this is generally the
// rule)
//
// Borrow with &, memory ownership will be borrowed for the procedure call
//
// Strings are always on the heap
// &str is a pointer... to either stack or heap
//
// You can have multiple references, (read only pointers) to memory as long as the owner does not
// change before the references are used
//
// #[derive(Clone)] macro for adding clone trait to allow struct to be cloned
// e.g.
// #[derive(Debug, Clone)]
// struct MyStruct {
//   a: i32,
//   b: f64,
// }
//
// Prefer & if possible over Clone & Copy traits
//
// Rust Ownership & Borrowing
// Rust eliminates memory issues (null pointers, dangling pointers, data races)
// Eliminates the garbage collector
// Parallel processing is a breeze!
//
// Keep in mind stack vs heap when dealing with ownership & borrowing

// Define a collection of commands that can be parsed from file contents
// implement an iterator that can run iterate the commands
// 3 command types A_COMMAND, C_COMMAND, L_COMMAND (pseudo command)

pub struct Parser<'a> {
    commands: Vec<Command<'a>>,
    current_command_idx: usize,
    // lines: Vec<String>,
    // current_line: u32,
    // current_command: Command,
}

// ACommand regex - @R?\d{1,}
// ACommand behind comment regex (/{2}.*)@R?\d{1,}
// c-instruction generated by grex
// ^(?:A?M(?:D=(?:(?:A\+1|D(?:[+\-][1A]|[&|]A)|![AD]|[01])(?:;J(?:EQ|[GL][ET]|MP|NE))?|A?\-[1D](?:;J(?:EQ|[GL][ET]|MP|NE))?|[AD];J(?:EQ|[GL][ET]|MP|NE)|[AD])|=(?:(?:A\+1|D(?:[+\-][1A]|[&|]A)|![AD]|[01])(?:;J(?:EQ|[GL][ET]|MP|NE))?|A?\-[1D](?:;J(?:EQ|[GL][ET]|MP|NE))?|[AD];J(?:EQ|[GL][ET]|MP|NE)|[AD]))|AD=(?:(?:A\+1|D(?:[+\-][1A]|[&|]A)|![AD]|[01])(?:;J(?:EQ|[GL][ET]|MP|NE))?|A?\-[1D](?:;J(?:EQ|[GL][ET]|MP|NE))?|[AD];J(?:EQ|[GL][ET]|MP|NE)|[AD])|[AD]=(?:(?:A\+1|D(?:[+\-][1A]|[&|]A)|![AD]|[01])(?:;J(?:EQ|[GL][ET]|MP|NE))?|A?\-[1D](?:;J(?:EQ|[GL][ET]|MP|NE))?|[AD];J(?:EQ|[GL][ET]|MP|NE)|[AD])|(?:A\+1|D[+\-][1A]|D[&|]A|![AD]|[01])(?:;J(?:EQ|[GL][ET]|MP|NE))?|A?\-[1D](?:;J(?:EQ|[GL][ET]|MP|NE))?|[AD];J(?:EQ|[GL][ET]|MP|NE)|[AD])$

fn get_command_type(command: &str) -> Option<Command> {
    let a_command_regex = Regex::new("@R?\\d{1,}").unwrap();
    let l_command_regex = Regex::new("\\(.*\\)").unwrap();

    let is_l_command = l_command_regex.is_match(command);
    let is_a_command = a_command_regex.is_match(command);

    if is_l_command {
        Some(Command::LCommand(command))
    } else if is_a_command {
        Some(Command::ACommand(command))
    } else {
        // perform c command validation here
        if command.is_empty() {
            None
        } else {
            Some(Command::CCommand(command))
        }
    }
}

impl Parser<'_> {
    // Reads the next command from the input & makes it the current command.
    // pub fn advance(&self) {
    //     // while let lines[&self.current_line]
    //     while &self.current_line > 0..&self.lines.len() {
    //         println!("{}", &self.lines[&self.current_line]) & self.current_line += 1
    //     }

    //     // look for next command
    //     ()
    // }

    // fn current_command(&self) {
    //     &self.commands[&self.current_command_idx];
    // }
    // pub fn has_more_commands() -> bool {}
    // pub fn advance(&self) -> Result<&Vec<u8>, Box<dyn Error>> {
    //     let command = &self.contents;
    // }

    // pub fn contents(&self) -> String {
    //     let contents =
    //         fs::read_to_string(&self.filename).expect("Something went wrong reading the file.");

    //     contents
    // }
}

#[derive(Clone)]
enum Command<'a> {
    ACommand(&'a str), // address
    CCommand(&'a str), // compute
    LCommand(&'a str), // pseudo
}

// impl Command {
//     fn new(line: String) -> Command {
//         Command { line }
//     }

//     // fn type {
// ez
//     // }
//     // fn symbol {
// use rust matching for these methods
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

    let contents = fs::read_to_string(filename).expect("no such file");
    let commands: Vec<Command> = Vec::new();

    // remove comments and whitespace, may still contain newlines
    let regex = Regex::new("\n{2,}|[^\\S\\r\\n]|//.*").unwrap();
    let contents = regex.replace_all(&contents, "").into_owned();

    let commands = contents
        .lines()
        .filter_map(|line| get_command_type(line))
        .collect::<Vec<Command>>();

    // let parser = Parser::new_from_file(filename);
    let parser = Parser {
        commands,
        current_command_idx: 0,
    };

    // parser.advance();
    for command in parser.commands {
        if let Command::ACommand(val) = command {
            println!("a-{:?}", val);
        }
        if let Command::LCommand(val) = command {
            println!("l-{:?}", val);
        }
        if let Command::CCommand(val) = command {
            println!("c-{:?}", val);
        }
    }

    Ok(())
}
