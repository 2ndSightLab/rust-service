mod security_checks {
    mod common {
        mod test_code_separation;
        mod test_dependency_audit;
        mod test_dependency_validation;
    }
    mod app {
        mod test_circular_dependency_prevention;
        mod test_directory_traversal;
        mod test_file_descriptor_leak;
        mod test_file_descriptor_race_conditions;
        mod test_input_sanitization;
        mod test_insecure_directories;
        mod test_integer_overflow_protection;
        mod test_libc_error_handling;
        mod test_service_user_validation;
        mod test_toctou_prevention;
        mod test_unsafe_libc_hardening;
    }
}
