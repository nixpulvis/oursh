use docopt::ArgvMap;
use crate::process::{Jobs, IO};
#[cfg(feature = "history")]
use crate::repl::history::History;

#[derive(Debug)]
pub struct Runtime<'a> {
    pub background: bool,
    pub io: IO,
    pub jobs: &'a mut Jobs,
    pub args: &'a mut ArgvMap,
    #[cfg(feature = "history")]
    pub history: &'a mut History,
}
