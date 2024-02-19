//! Commands that are run from the shell directly, without forking another
//! process.
//!
//! These commands take precedence over any executables with the same name
//! in the `$PATH`.
use crate::program::{Result, Runtime};
use nix::sys::wait::WaitStatus;
use std::ffi::CString;

/// A builtin is a custom shell command, often changing the state of the
/// shell in some way.
pub trait Builtin {
    /// Execute the shell builtin command, returning a retult of the
    /// completion.
    fn run(self, argv: Vec<CString>, runtime: &mut Runtime) -> Result<WaitStatus>;
}

mod cd;
pub use self::cd::Cd;
mod command;
pub use self::command::Command;
mod dot;
pub use self::dot::Dot;
mod exit;
pub use self::exit::Exit;
mod export;
pub use self::export::Export;
mod jobs;
pub use self::jobs::Jobs;
mod r#return;
pub use self::r#return::Return;
mod wait;
pub use self::wait::Wait;
