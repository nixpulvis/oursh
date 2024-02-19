use crate::process::{Jobs, IO};
#[cfg(feature = "history")]
use crate::repl::history::History;
use docopt::ArgvMap;

#[derive(Debug)]
pub struct Runtime<'a> {
    pub background: bool,
    pub io: IO,
    pub jobs: &'a mut Jobs,
    pub args: &'a ArgvMap,
    #[cfg(feature = "history")]
    pub history: &'a mut History,
}
