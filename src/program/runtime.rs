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
    /// The REPL's history, `None` when not running interactively.
    #[cfg(feature = "history")]
    pub history: Option<&'a mut History>,
}
