use std::io::Read;
use std::ffi::CString;
use pest::Parser;

#[derive(Parser)]
#[grammar = "program/simple.pest"]
pub struct SimpleParser;

#[derive(Debug)]
pub struct SimpleProgram(Vec<Command>);
#[derive(Debug)]
struct Command(String);

impl super::Parser for SimpleProgram {
    type Target = SimpleProgram;

    fn parse<R: Read>(mut reader: R) -> Self {
        let mut string = String::new();
        reader.read_to_string(&mut string);
        println!("input: {:?}", string);
        let parsed = SimpleParser::parse(Rule::program, &string)
            .unwrap().next().unwrap();
        let commands = parsed.into_inner().map(|pair| {
            Command(pair.as_str().into())
        }).collect();
        println!("output: {:#?}", commands);
        SimpleProgram(commands)
    }
}

impl super::Program for SimpleProgram {
    fn argv(&self) -> Vec<CString> {
        self.0[0].0.split(' ').map(|s| CString::new(s).unwrap()).collect()
    }
}
