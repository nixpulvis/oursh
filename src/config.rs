//! WTF!
//!
//!
//!
use std::{
    fs::File,
    io::Read,
};
use rustyline::Editor;
use docopt::ArgvMap;
use crate::{
    program::parse_and_run,
    process::{Jobs, IO},
};

pub struct Runtime {
    pub io: IO,
    pub jobs: Jobs,
    pub args: ArgvMap,
    pub rl: Option<Editor<()>>,
}

/// TODO
// TODO: Use the builtin `source` command when it's written.
pub fn source_profile(runtime: &mut Runtime) {
    if let Some(mut path) = dirs::home_dir() {
        path.push(".oursh_profile");
        if let Ok(mut file) = File::open(path) {
            let mut contents = String::new();
            if let Ok(_) = file.read_to_string(&mut contents) {
                if let Err(e) = parse_and_run(&contents, runtime.io, &mut runtime.jobs, &runtime.args, runtime.rl.as_mut()) {
                    eprintln!("failed to source profile: {:?}", e);
                }
            }
        }
    }
}
