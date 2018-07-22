extern crate oursh;

use oursh::job::Job;
use oursh::program::Program;

fn main() {
    Job::new(&Program::parse(b"ls" as &[u8])).run();
    Job::new(&Program::parse(b"date --iso-8601" as &[u8])).run();
    Job::new(&Program::parse(b"error" as &[u8])).run();
}
