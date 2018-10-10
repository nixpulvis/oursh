extern crate oursh;

use oursh::job::Job;
use oursh::program::parse;

fn main() {
    Job::new(&parse(b"ls" as &[u8])).run();
    Job::new(&parse(b"date --iso-8601" as &[u8])).run();
    Job::new(&parse(b"error" as &[u8])).run();
}
