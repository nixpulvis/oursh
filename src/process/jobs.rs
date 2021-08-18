use std::{
    rc::Rc,
    cell::RefCell,
};
use nix::sys::wait::WaitStatus;
use retain_mut::RetainMut;
use crate::process::{Wait, ProcessGroup};

/// Shared job handling structure
///
/// Maintains a collection of process groups.
// TODO: Make into slightly better struct.
pub type Jobs = Rc<RefCell<Vec<(String, ProcessGroup)>>>;

/// Enumerate the given jobs, pruning exited, signaled or otherwise errored process groups
pub fn retain_alive(jobs: &mut Jobs) {
    jobs.borrow_mut().retain_mut(|job| {
        let id = job.0.clone();
        let body = job.1.leader().body();
        match job.1.leader().status() {
            Ok(WaitStatus::StillAlive) => {
                true
            },
            Ok(WaitStatus::Exited(pid, code)) => {
                println!("[{}]+\tExit({})\t{}\t{}", id, code, pid, body);
                false
            },
            Ok(WaitStatus::Signaled(pid, signal, _)) => {
                println!("[{}]+\t{}\t{}\t{}", id, signal, pid, body);
                false
            },
            Ok(_) => {
                println!("unhandled");
                true
            },
            Err(e) => {
                if nix::errno::Errno::ECHILD != e {
                    println!("err: {:?}", e);
                }

                false
            }
        }
    });
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn retain_alive_removes_jobs() {}
    #[test]
    #[ignore]
    fn retain_alive_prints_job_status() {}
}
