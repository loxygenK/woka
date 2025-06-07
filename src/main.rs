use std::process::ExitCode;

pub mod accept;
pub mod apps;
pub mod config;
pub mod log;
pub mod ssh;

fn main() -> ExitCode {
    accept::parse_and_run().unwrap()
}
