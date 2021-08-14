use std::ffi::CString;
use nix::{
    unistd::Pid,
    sys::wait::WaitStatus,
};
use crate::{
    program::posix::builtin::Builtin,
    program::{Result, Error, Runtime},
    process::Wait as WaitTrait,
};

/// Wait builtin, used to block for all background jobs.
pub struct Wait;

impl Builtin for Wait {
    fn run(self, argv: Vec<CString>, runtime: &mut Runtime) -> Result<WaitStatus> {
        match argv.len() {
            0 => unreachable!(),
            1 => {
                let mut last = Ok(WaitStatus::Exited(Pid::this(), 0));
                for job in runtime.jobs.borrow().iter() {
                    last = job.1.leader().wait().map_err(|_| Error::Runtime)
                }
                last
            }
            n => {
                let mut last = Ok(WaitStatus::Exited(Pid::this(), 0));
                for i in 2..n {
                    let pid: i32 = argv[i-1].to_string_lossy().parse().unwrap();
                    if let Some((_id, pg)) = runtime.jobs.borrow().iter().find(|(_, pg)| {
                        pid == pg.leader().pid().as_raw()
                    }) {
                        last = pg.leader().wait().map_err(|_| Error::Runtime)
                    }
                }
                last
            },
        }
    }
}
