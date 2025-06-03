pub mod accept;
pub mod apps;
pub mod config;

fn main() {
    accept::parse_and_run().unwrap();
}
