use std::io::Read;
use std::ffi::CString;
use pest::Parser;

#[derive(Parser)]
#[grammar = "program/simple.pest"]
struct SimpleParser;

pub struct SimpleProgram;

impl super::Parser for SimpleProgram {
    type Target = SimpleProgram;

    fn parse<R: Read>(mut reader: R) -> Self {
        SimpleProgram
    }
}

impl super::Program for SimpleProgram {
    fn argv(&self) -> Vec<CString> {
        vec![CString::new("ls").unwrap()]
    }
}
