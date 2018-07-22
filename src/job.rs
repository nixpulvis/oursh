use std::ffi::CString;
use std::process::exit;
use nix::unistd::{execvp, fork, Pid, ForkResult};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::Error;
use nix::errno::Errno;

pub struct Job {
    // TODO: Use a program type.
    argv: Vec<CString>,
    child: Option<Pid>,
}

impl Job {
    // TODO: Return result.
    pub fn new(args: &str) -> Self {
        // TODO: Proper parsing needed, as this will take a `Program`.
        let vec = args.split_whitespace().map(|a| {
            CString::new(a).expect("error reading string argument")
        }).collect();

        Job {
            argv: vec,
            child: None,
        }
    }

    // TODO: Return result.
    pub fn run(&mut self) {
        // TODO: Proper builtins, in program module.
        if self.argv[0].to_bytes() == b"exit" {
            exit(0);
        }

        self.fork_and_wait();
    }

    fn fork(&mut self) {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child = Some(child);
            },
            Ok(ForkResult::Child) => {
                self.exec();
            },
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }

    fn fork_and_wait(&mut self) {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child = Some(child);
                self.wait();
            },
            Ok(ForkResult::Child) => {
                self.exec();
            },
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }

    fn exec(&self) {
        match execvp(&self.argv[0], &self.argv) {
            Ok(_) => unreachable!(),
            Err(Error::Sys(e @ _)) => println!("error: {}", e.desc()),
            _ => {}
        }
    }

    fn wait(&self) {
        match self.child {
            Some(child) => {
                loop {
                    match waitpid(child, None) {
                        Ok(WaitStatus::StillAlive) => {},
                        // TODO: Cover other cases?
                        _ => break,
                    }
                }
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO
}
