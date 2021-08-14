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
        let children = &mut job.1.leader_mut().children;
        let id = job.0.clone();

        children.retain_mut(|child| {
            match child.status() {
                Ok(WaitStatus::StillAlive) => {
                    true
                },
                Ok(WaitStatus::Exited(pid, code)) => {
                    println!("[{}]+\tExit({})\t{}", id, code, pid);
                    false
                },
                Ok(WaitStatus::Signaled(pid, signal, _)) => {
                    println!("[{}]+\t{}\t{}", id, signal, pid);
                    false
                },
                Ok(_) => {
                    println!("unhandled");
                    true
                },
                Err(e) => {
                    println!("err: {:?}", e);
                    false
                }
            }
        });

        !children.is_empty()
    });
}
