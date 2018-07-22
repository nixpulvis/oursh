extern crate oursh;

use oursh::job::Job;

fn main() {
    Job::new("ls").run();
    Job::new("date --iso-8601").run();
    Job::new("error").run();
}
