use std::process::ExitCode;

pub mod accept;
pub mod connect;
pub mod config;
pub mod log;

fn main() -> ExitCode {
    accept::parse_and_run().unwrap()
}
