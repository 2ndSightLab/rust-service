mod common;

mod integration {
    pub mod app {
        pub mod test_config_discovery;
        pub mod test_graceful_shutdown_handling;
        pub mod test_logging_integration;
        pub mod test_monitoring_integration;
        pub mod test_prerequisites;
        pub mod test_prerequisites_check;
        pub mod test_security_workflow;
        pub mod test_security_workflow_integration;
        pub mod test_service_lifecycle;
        pub mod test_shutdown_handling;
        pub mod test_system_resource_monitoring;
    }
}

#[test]
fn run_all_common_integration_tests() {
    common::run_common_tests("integration_common");
}
