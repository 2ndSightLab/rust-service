mod integration {
    mod app {
        mod config_discovery_test;
        mod logging_integration_test;
        mod service_lifecycle_test;
    }
    mod common {
        mod monitoring_integration_test;
        mod security_workflow_test;
        mod shutdown_handling_test;
        pub mod test_prerequisites;
    }
}
