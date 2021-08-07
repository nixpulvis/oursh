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

use std::{
    borrow::Cow,
    process::exit,
    ffi::CString,
    cell::RefCell,
    rc::Rc,
    os::unix::io::RawFd,
};
use docopt::ArgvMap;
use nix::{
    unistd::{self, execvp, dup2, close, Pid, ForkResult},
    sys::wait::{waitpid, WaitStatus, WaitPidFlag},
};
use crate::program::{Run, parse_primary, parse_alternate};

/// TODO: Rename `Job` to `Process`
struct Process;

/// TODO: Syntax for this?
/// TODO: Differences between &?
struct Thread;

/// TODO: What signal handling can we put here?
struct Signal;

/// Processes groups are used for things like pipelines and background jobs. The system call `int
/// setpgid(pid_t pid, pid_t pgid)` is used to set.
struct ProcessGroup;

/// The session's ID is the same as the pid of the process that created the session through the
/// `setsid` system call. That process is known as the _session leader_ for that session group. All
/// of that process's descendants are then members of that session unless they specifically remove
/// themselves from it.
struct Session;


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

/// A job to be executed by various means.
///
/// The shell's main job (pun intended) is to run commands. Each job has various arguments, and
/// rules about what things should be done.
///
/// - TODO #4: Redirection example.
/// - TODO #6: Background example.
/// - TODO #4: Environment example?
///
/// TODO: Major flaw! Jobs need to be more than a single command's execution
/// parameters. Jobs for example can be backgrounded on compound (etc.) types,
/// `{ echo 1; sleep 2; }&` or `echo 1 hello world | wc &`. Each should be
/// exactly **one** Job each.
pub struct Job {
    argv: Vec<CString>,
    // TODO: Call this pid?
    child: Option<Pid>,
}

impl Job {
    /// Create a new job from the given command.
    // TODO #4: Return result.
    pub fn new(argv: Vec<CString>) -> Self {
        Job {
            argv,
            child: None,
        }
    }

    pub fn body(&self) -> String {
        self.argv.iter().map(|a| {
            a.to_string_lossy()
        }).collect::<Vec<Cow<str>>>().join(" ")
    }

    pub fn pid(&self) -> Option<Pid> {
        self.child
    }

    pub fn status(&self) -> nix::Result<WaitStatus> {
        match self.child {
            Some(child) => {
                waitpid(child, Some(WaitPidFlag::WNOHANG))
            },
            _ => unimplemented!(),
        }
    }

    /// Run a shell job in the background.
    pub fn fork(&mut self, io: IO) -> nix::Result<WaitStatus> {
        match unistd::fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child = Some(child);
                self.status()
            },
            Ok(ForkResult::Child) => {
                io.dup()?;
                // TODO #20: When running with raw mode we could buffer
                // this and print it later, all at once in suspended raw mode.
                if let Err(_) = self.exec() {
                    exit(127);
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
                self.child = Some(child);
                self.wait()
            },
            Ok(ForkResult::Child) => {
                io.dup()?;
                if let Err(_) = self.exec() {
                    exit(127);
                } else {
                    // TODO: Waiting in the child?
                    unimplemented!();
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

    fn wait(&mut self) -> nix::Result<WaitStatus> {
        match self.child {
            Some(child) => {
                loop {
                    match waitpid(child, None) {
                        // TODO #4: Cover other cases?
                        Ok(WaitStatus::StillAlive) => {},
                        s @ Ok(WaitStatus::Exited(_, 127)) => {
                            let name = self.argv[0].to_string_lossy();
                            eprintln!("oursh: {}: command not found", name);
                            return s;
                        },
                        s => return s,
                    };
                }
            },
            _ => unimplemented!(),
        }
    }
}

// TODO: Make into slightly better struct.
pub type Jobs = Rc<RefCell<Vec<(String, Job)>>>;

// TODO: Replace program::Result
pub fn retain_alive_jobs(jobs: &mut Jobs) -> crate::program::Result<()> {
    jobs.borrow_mut().retain(|job| {
        match job.1.status() {
            Ok(WaitStatus::StillAlive) => {
                true
            },
            Ok(WaitStatus::Exited(pid, code)) => {
                println!("[{}]+\tExit({})\t{}", job.0, code, pid);
                false
            },
            Ok(WaitStatus::Signaled(pid, signal, _)) => {
                println!("[{}]+\t{}\t{}", job.0, signal, pid);
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

    Ok(())
}
