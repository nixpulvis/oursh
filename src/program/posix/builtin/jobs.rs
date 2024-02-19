use crate::{
    program::posix::builtin::Builtin,
    program::{Result, Runtime},
};
use nix::{sys::wait::WaitStatus, unistd::Pid};
use std::ffi::CString;

/// Background job information.
pub struct Jobs;

impl Builtin for Jobs {
    fn run(self, _: Vec<CString>, runtime: &mut Runtime) -> Result<WaitStatus> {
        for (id, job) in runtime.jobs.borrow().iter() {
            println!(
                "[{}]\t{}\t\t{}",
                id,
                job.leader().pid(),
                job.leader().body()
            );
        }
        Ok(WaitStatus::Exited(Pid::this(), 0))
    }
}
