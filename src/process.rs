//! Subprocess execution management.
//!
//! All commands (both foreground and background) are created and executed as
//! a *job*. This helps manage the commands the shell runs.
//!
//! - Job control
//! - Threads?
//! - Signals
//! - Process groups
//! - Sessions
//!
//! More resources from the source at:
//! [www.win.tue.nl/~aeb/linux/lk/lk-10](https://www.win.tue.nl/~aeb/linux/lk/lk-10.html).

use std::{
    borrow::Cow,
    process::exit,
    ffi::CString,
    cell::RefCell,
    rc::Rc,
    os::unix::io::RawFd,
};
use nix::{
    errno::Errno,
    unistd::{self, execvp, dup2, close, getpid, setsid, Pid, ForkResult},
    sys::termios::{tcgetattr, tcsetattr, SetArg, OutputFlags},
    sys::wait::{waitpid, WaitStatus, WaitPidFlag},
};
use retain_mut::RetainMut;

/// File descriptors for use in processes and threads
#[derive(Debug, Copy, Clone)]
pub struct IO(pub [RawFd; 3]);

impl IO {
    fn dup(&self) -> Result<(), nix::Error> {
        if self.0[0] != 0 {
            dup2(self.0[0], 0)?;
            close(self.0[0])?;
        }
        if self.0[1] != 1 {
            dup2(self.0[1], 1)?;
            close(self.0[1])?;
        }
        if self.0[2] != 2 {
            dup2(self.0[2], 2)?;
            close(self.0[2])?;
        }
        Ok(())
    }
}

impl Default for IO {
    fn default() -> Self {
        // [stdin, stdout, stderr]
        IO([0, 1, 2])
    }
}

/// A process to be executed by various means
///
/// The shell's main job is to run commands. Each job has various arguments, and rules about what
/// things should be done.
///
/// - TODO #4: Redirection example.
/// - TODO #6: Background example.
/// - TODO #4: Environment example?
#[derive(Debug)]
pub struct Process {
    argv: Vec<CString>,
    pid: Pid,
    children: Vec<Pid>,
}

impl Process {
    /// Create a new job from the given command.
    // TODO #4: Return result.
    pub fn new(argv: Vec<CString>) -> Self {
        Process {
            argv,
            pid: getpid(),
            children: Vec::new(),
        }
    }

    pub fn body(&self) -> String {
        self.argv.iter().map(|a| {
            a.to_string_lossy()
        }).collect::<Vec<Cow<str>>>().join(" ")
    }

    pub fn pid(&self) -> Pid {
        self.pid
    }

    /// Run a shell job in the background.
    pub fn fork(&mut self, io: IO) -> nix::Result<WaitStatus> {
        match unistd::fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.children.push(child);
                child.status()
            },
            Ok(ForkResult::Child) => {
                io.dup()?;
                // dbg!(&io);
                // self.pid = setsid()?;
                // TODO #20: When running with raw mode we could buffer
                // this and print it later, all at once in suspended raw mode.

                if let Ok(mut term) = tcgetattr(io.0[1]) {
                    dbg!(&term);
                    term.output_flags |= OutputFlags::ONLCR;
                    term.output_flags |= OutputFlags::NLDLY;
                    tcsetattr(io.0[1], SetArg::TCSANOW, &term);
                }

                // if let Ok(mut term) = tcgetattr(io.0[1]) {
                //     // term.output_flags &= !OutputFlags::OCRNL;
                //     term.output_flags &= !OutputFlags::ONLCR;
                //     tcsetattr(io.0[1], SetArg::TCSANOW, &term);
                // }
                // if let Ok(mut term) = tcgetattr(2) {
                //     term.output_flags &= !OutputFlags::OCRNL;
                //     tcsetattr(2, SetArg::TCSANOW, &term);
                // }

                if let Err(e) = self.exec() {
                    match e {
                        nix::Error::Sys(Errno::ENOENT) => {
                            let name = self.argv[0].to_string_lossy();
                            eprintln!("oursh: {}: command not found", name);
                            exit(127);
                        },
                        _ => exit(128),
                    }
                } else {
                    self.status()
                }
            },
            Err(e) => Err(e),
        }
    }

    /// Run a shell job, waiting for the command to finish.
    pub fn fork_and_wait(&mut self, io: IO) -> nix::Result<WaitStatus> {
        match unistd::fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.children.push(child);
                let status = child.wait();
                match status {
                    Ok(WaitStatus::Exited(_, 127)) => {
                        let name = self.argv[0].to_string_lossy();
                        eprintln!("oursh: {}: command not found", name);
                    },
                    _ => {}
                }
                status
            },
            Ok(ForkResult::Child) => {
                io.dup()?;
                if let Err(_) = self.exec() {
                    exit(127);
                } else {
                    self.wait()
                }
            },
            Err(e) => Err(e),
        }
    }

    fn exec(&self) -> Result<(), nix::Error> {
        execvp(&self.argv[0], &self.argv.iter()
                                        .map(|a| a.as_c_str())
                                        .collect::<Vec<_>>()[..]).map(|_| ())
    }
}

pub trait Wait {
    fn wait(&self) -> nix::Result<WaitStatus>;
    fn status(&self) -> nix::Result<WaitStatus>;
}

impl Wait for Pid {
    fn wait(&self) -> nix::Result<WaitStatus> {
        waitpid(Some(*self), None)
    }

    fn status(&self) -> nix::Result<WaitStatus> {
        waitpid(Some(*self), Some(WaitPidFlag::WNOHANG))
    }
}

impl Wait for Process {
    fn wait(&self) -> nix::Result<WaitStatus> {
        self.pid.wait()
    }

    fn status(&self) -> nix::Result<WaitStatus> {
        self.pid.status()
    }
}

/// Threads are distinguished by a thread ID (TID)
///
/// An ordinary process has a single thread with TID equal to PID.
///
/// TODO: Syntax for this?
/// TODO: Differences between &?
pub struct Thread;

/// TODO: What signal handling can we put here?
pub struct Signal;

/// Processes groups are used for things like pipelines and background jobs
///
/// The system call `int setpgid(pid_t pid, pid_t pgid)` is used to set.
///
/// Every process is member of a unique process group, identified by its process group ID. (When
/// the process is created, it becomes a member of the process group of its parent.) By convention,
/// the process group ID of a process group equals the process ID of the first member of the
/// process group, called the process group leader. A process finds the ID of its process group
/// using the system call getpgrp(), or, equivalently, getpgid(0). One finds the process group ID
/// of process p using getpgid(p).
#[derive(Debug)]
pub struct ProcessGroup(pub Process); // TODO: Make a sorted vector of Process?

impl ProcessGroup {
    pub fn leader(&self) -> &Process {
        &self.0
    }

    pub fn leader_mut(&mut self) -> &mut Process {
        &mut self.0
    }
}


/// Shared job handling structure
///
/// Maintains a collection of process groups.
// TODO: Make into slightly better struct.
pub type Jobs = Rc<RefCell<Vec<(String, ProcessGroup)>>>;

/// Enumerate the given jobs, pruning exited, signaled or otherwise errored process groups
// TODO: Replace program::Result
pub fn retain_alive_jobs(jobs: &mut Jobs) {
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


/// Every process group is in a unique session.
///
/// (When the process is created, it becomes a member of the session of its parent.) By convention,
/// the session ID of a session equals the process ID of the first member of the session, called
/// the session leader. A process finds the ID of its session using the system call getsid().
///
/// Every session may have a controlling tty, that then also is called the controlling tty of each
/// of its member processes. A file descriptor for the controlling tty is obtained by opening
/// /dev/tty. (And when that fails, there was no controlling tty.) Given a file descriptor for the
/// controlling tty, one may obtain the SID using tcgetsid(fd).
struct Session;
