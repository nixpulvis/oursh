//! WTF!
//!
//!
//!
use crate::program::{
    posix::builtin::{self, Builtin},
    Runtime,
};
use std::ffi::CString;

/// Sourcing profile startup scripts
///
/// For now we just load `.oursh_profile`.
// TODO: Use the builtin `source` command when it's written.
pub fn source_profile(runtime: &mut Runtime) {
    if let Some(mut path) = dirs::home_dir() {
        path.push(".oursh_profile");
        let argv = vec![
            CString::new("source".to_string()).unwrap(),
            CString::new(path.to_str().unwrap()).expect("valid path string"),
        ];
        if let Err(e) = builtin::Dot.run(argv, runtime) {
            eprintln!("failed to source profile: {:?}", e);
        }
    }
}
