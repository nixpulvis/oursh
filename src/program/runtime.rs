use rustyline::Editor;
use docopt::ArgvMap;
use crate::process::{Jobs, IO};

#[derive(Debug)]
pub struct Runtime<'a> {
    pub background: bool,
    pub io: IO,
    pub jobs: &'a mut Jobs,
    pub args: &'a mut ArgvMap,
    pub rl: Option<&'a mut Editor<()>>,
}
