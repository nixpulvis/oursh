//! WTF!
//!
//!
//!
use std::ffi::CString;
use crate::program::{
    Runtime,
    posix::builtin::{self, Builtin},
};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn source_profile_login() {}
    #[test]
    #[ignore]
    fn source_profile_interactive() {}
    #[test]
    #[ignore]
    fn source_profile_noprofile() {}
    #[test]
    #[ignore]
    fn source_profile_nologin() {}
}
