use std::os::unix::io::RawFd;
use nix::{
    unistd::{dup2, close},
};

/// File descriptors for use in processes and threads
#[derive(Debug, Copy, Clone)]
pub struct IO(pub [RawFd; 3]);

impl IO {
    pub fn dup(&self) -> Result<(), nix::Error> {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn io_default() {}
    #[test]
    #[ignore]
    fn io_pipe() {}
    #[test]
    #[ignore]
    fn io_dup() {}
}
