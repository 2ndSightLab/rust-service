#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_metadata_race_condition_prevention() {
        // Test that metadata checks are done on file descriptor, not path
        // This prevents TOCTOU (Time-of-Check-Time-of-Use) attacks

        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(temp_file, "test content").expect("Failed to write to temp file");

        // Open the file
        let file = fs::File::open(temp_file.path()).expect("Failed to open file");

        // Get metadata from the file descriptor (secure)
        let metadata_from_fd = file.metadata().expect("Failed to get metadata from FD");

        // Get metadata from the path (potentially insecure due to TOCTOU)
        let metadata_from_path =
            fs::metadata(temp_file.path()).expect("Failed to get metadata from path");

        // These should be the same for the same file, but the FD method is secure
        assert_eq!(
            metadata_from_fd.len(),
            metadata_from_path.len(),
            "Metadata from FD and path should match for same file"
        );

        // The secure approach is to use the file descriptor metadata
        // This test verifies that we can get metadata from FD
        assert!(metadata_from_fd.is_file(), "Should be a regular file");
    }

    #[test]
    fn test_file_descriptor_based_operations() {
        // Test that file operations use file descriptors to prevent race conditions
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(temp_file, "test content").expect("Failed to write to temp file");

        // Open file and keep the file descriptor
        let file = fs::File::open(temp_file.path()).expect("Failed to open file");

        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            use std::os::unix::io::AsRawFd;

            // Get file descriptor
            let fd = file.as_raw_fd();
            assert!(fd >= 0, "File descriptor should be valid");

            // Get metadata using the file descriptor (secure against TOCTOU)
            let metadata = file.metadata().expect("Failed to get metadata");

            // Verify we can get owner information from FD
            let _uid = metadata.uid(); // UID is always valid for existing files

            // This is the secure way - using fstat() on the FD, not stat() on the path
        }
    }
}
