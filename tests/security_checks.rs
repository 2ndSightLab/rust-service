mod security_checks {
    mod test_file_descriptor_leak;
    mod test_directory_traversal;
    mod test_input_sanitization;
    mod test_service_user_validation;
    mod test_libc_error_handling;
    mod test_toctou_prevention;
}
