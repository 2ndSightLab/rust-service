mod common;

mod unit {
    pub mod app {
        pub mod config_tests;
        pub mod monitoring_tests;
        pub mod monitoring_unit_tests;
        pub mod test_config_standards;
        pub mod test_toml_lint;
        pub mod test_variable_naming;
        pub mod test_best_practices;
        pub mod test_script_validation;
    }
}

#[test]
fn run_all_common_unit_tests() {
    common::run_common_tests("unit_tests_common");
}
