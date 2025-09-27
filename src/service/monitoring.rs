use crate::service::config::Config;
use crate::service::error::ServiceError;
use log::warn;
use std::fs;

/// Checks system memory and disk usage against configured thresholds.
///
/// # Errors
/// Returns `ServiceError::Config` if system resources exceed configured thresholds.
pub fn check_resources(CONFIG: &Config) -> Result<(), ServiceError> {
    check_memory(CONFIG.MEMORY_THRESHOLD)?;
    check_disk(CONFIG.DISK_THRESHOLD)?;
    Ok(())
}

fn calculate_usage_percent(
    USED: u64,
    TOTAL: u64,
    RESOURCE_TYPE: &str,
) -> Result<u64, ServiceError> {
    USED.checked_mul(100)
        .and_then(|X| X.checked_div(TOTAL))
        .ok_or_else(|| ServiceError::Config(format!("{RESOURCE_TYPE} calculation overflow")))
}

fn safe_subtract(TOTAL: u64, AVAILABLE: u64, RESOURCE_TYPE: &str) -> Result<u64, ServiceError> {
    TOTAL
        .checked_sub(AVAILABLE)
        .ok_or_else(|| ServiceError::Config(format!("Invalid {RESOURCE_TYPE} values")))
}

fn check_threshold_and_error(
    USAGE_PERCENT: u64,
    THRESHOLD: u32,
    RESOURCE_TYPE: &str,
) -> Result<(), ServiceError> {
    if USAGE_PERCENT >= u64::from(THRESHOLD) {
        let ERROR_MSG = format!("{RESOURCE_TYPE} usage >= {THRESHOLD}%");
        warn!("{ERROR_MSG}");
        return Err(ServiceError::Config(ERROR_MSG));
    }
    Ok(())
}

fn check_memory(THRESHOLD: u32) -> Result<(), ServiceError> {
    let MEMINFO = fs::read_to_string("/proc/meminfo").map_err(|e| {
        ServiceError::Config(format!("Cannot read memory info from /proc/meminfo: {e}"))
    })?;

    let mut TOTAL = None;
    let mut AVAILABLE = None;

    for LINE in MEMINFO.lines() {
        if let Some(VALUE_STR) = LINE
            .strip_prefix("MemTotal:")
            .and_then(|S| S.split_whitespace().next())
        {
            TOTAL = VALUE_STR.parse::<u64>().ok();
        } else if let Some(VALUE_STR) = LINE
            .strip_prefix("MemAvailable:")
            .and_then(|S| S.split_whitespace().next())
        {
            AVAILABLE = VALUE_STR.parse::<u64>().ok();
        }
    }

    let TOTAL = TOTAL.ok_or_else(|| ServiceError::Config("Memory info unavailable".to_string()))?;
    let AVAILABLE =
        AVAILABLE.ok_or_else(|| ServiceError::Config("Memory info unavailable".to_string()))?;

    if TOTAL > 0 {
        let USED = safe_subtract(TOTAL, AVAILABLE, "memory")?;
        let USAGE_PERCENT = calculate_usage_percent(USED, TOTAL, "Memory")?;
        check_threshold_and_error(USAGE_PERCENT, THRESHOLD, "Memory")?;
    }
    Ok(())
}

fn check_disk(THRESHOLD: u32) -> Result<(), ServiceError> {
    #[cfg(unix)]
    {
        use nix::sys::statvfs::statvfs;
        use std::path::Path;

        let STATS = statvfs(Path::new("/"))
            .map_err(|e| ServiceError::Config(format!("Cannot get disk statistics: {e}")))?;

        let TOTAL = STATS
            .blocks()
            .checked_mul(STATS.fragment_size())
            .ok_or_else(|| ServiceError::Config("Disk total calculation overflow".to_string()))?;
        let AVAILABLE = STATS
            .blocks_available()
            .checked_mul(STATS.fragment_size())
            .ok_or_else(|| {
                ServiceError::Config("Disk available calculation overflow".to_string())
            })?;

        if TOTAL > 0 && AVAILABLE <= TOTAL {
            let USED = safe_subtract(TOTAL, AVAILABLE, "disk")?;
            let USAGE_PERCENT = calculate_usage_percent(USED, TOTAL, "Disk")?;
            check_threshold_and_error(USAGE_PERCENT, THRESHOLD, "Disk")?;
        }
    }
    Ok(())
}
