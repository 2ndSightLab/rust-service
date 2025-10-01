mod common;

mod integration {
    pub mod app {
        pub mod config_discovery_test;
        pub mod logging_integration_test;
        pub mod monitoring_integration_test;
        pub mod security_workflow_test;
        pub mod service_lifecycle_test;
        pub mod shutdown_handling_test;
        pub mod test_prerequisites;
        pub mod test_graceful_shutdown_handling;
        pub mod test_prerequisites_check;
        pub mod test_security_workflow_integration;
        pub mod test_system_resource_monitoring;
    }
}

#[test]
fn run_all_common_integration_tests() {
    common::run_common_tests("integration_common");
}
