//! Barebones (pre-POSIX) program syntax.
use std::io::Read;
use std::ffi::CString;
use pest::Parser;

/// A program as parsed by a very simple, and incomplete PEG parser.
#[derive(Debug)]
pub struct SimpleProgram(Vec<Command>);

#[derive(Debug)]
struct Command(String);

impl super::Program for SimpleProgram {
    fn parse<R: Read>(mut reader: R) -> Self {
        let mut string = String::new();
        reader.read_to_string(&mut string).unwrap();
        println!("input: {:?}", string);
        let parsed = SimpleParser::parse(Rule::program, &string)
            .unwrap().next().unwrap();
        let commands = parsed.into_inner().map(|pair| {
            Command(pair.as_str().into())
        }).collect();
        println!("output: {:#?}", commands);
        SimpleProgram(commands)
    }

    fn commands(&self) -> Vec<super::Command> {
        self.0.iter().map(|c| {
            c.0.split(' ').map(|s| CString::new(s).unwrap()).collect()
        }).collect()
    }
}

#[derive(Parser)]
#[grammar = "program/simple.pest"]
struct SimpleParser;
