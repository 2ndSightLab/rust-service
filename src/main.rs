use log::info;
use rust_service::config::Config;
use rust_service::{Action, ServiceError, ServiceRunner};

struct MessageAction;

impl Action for MessageAction {
    fn execute(&self, config: &Config) -> Result<(), ServiceError> {
        info!("{}", config.MESSAGE);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "message"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ServiceRunner::new()
        .add_action(Box::new(MessageAction))
        .run()
}
