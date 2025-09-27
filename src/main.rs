pub mod action;
pub mod security;
pub mod service;

pub use action::exec;
pub use service::Config;

use service::ServiceRunner;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ServiceRunner::<Config>::new()
        .add_action(exec::new()?)
        .run()
}
