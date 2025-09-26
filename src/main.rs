use rust_service::{Action, ServiceRunner, ServiceError};
use rust_service::config::Config;
use log::info;

struct MessageAction;

impl Action for MessageAction {
    fn execute(&self, config: &Config) -> Result<(), ServiceError> {
        info!("{}", config.MESSAGE);
        Ok(())
    }

    fn name(&self) -> &str {
        "message"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ServiceRunner::new()
        .add_action(Box::new(MessageAction))
        .run()
}


