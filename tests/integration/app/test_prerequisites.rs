use rust_service::service::config_reader::load_config;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub struct TestPaths {
    pub binary_path: String,
    pub config_path: String,
}

pub fn get_test_paths() -> Result<TestPaths, String> {
    // Use application's config loading function
    let config = load_config().map_err(|e| format!("Failed to load config: {e}"))?;

    // Extract paths from loaded config
    let install_dir = &config.INSTALL_DIR;
    let config_dir = &config.CONFIG_DIR;

    // Check if debug installation exists, otherwise use release
    let debug_binary = format!("{install_dir}-debug/test-rust-service");
    let debug_config = format!("{config_dir}-debug/service.toml");

    let (binary_path, config_path) = if Path::new(&debug_binary).exists() {
        (debug_binary, debug_config)
    } else {
        (
            format!("{install_dir}/test-rust-service"),
            format!("{config_dir}/service.toml"),
        )
    };

    Ok(TestPaths {
        binary_path,
        config_path,
    })
}

pub fn check_debug_installation() -> Result<(), String> {
    let paths = get_test_paths()?;

    // Check if binary exists
    if fs::metadata(&paths.binary_path).is_err() {
        return Err(format!(
            "Binary not found at {}. Run './actions/test.sh' to install it.",
            paths.binary_path
        ));
    }

    // Check if config exists
    if fs::metadata(&paths.config_path).is_err() {
        return Err(format!(
            "Config not found at {}. Run './actions/test.sh' to install it.",
            paths.config_path
        ));
    }

    // Check if binary is executable
    let metadata = fs::metadata(&paths.binary_path)
        .map_err(|e| format!("Cannot access binary {}: {e}", paths.binary_path))?;

    let permissions = metadata.permissions();
    if permissions.mode() & 0o111 == 0 {
        return Err(format!("Binary {} is not executable", paths.binary_path));
    }

    Ok(())
}

#[test]
fn test_debug_installation_exists() {
    if let Err(msg) = check_debug_installation() {
        panic!("Integration test prerequisites not met: {msg}");
    }
}
