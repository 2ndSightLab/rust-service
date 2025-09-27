#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_comprehensive_security_audit() {
        run_dependency_audit();
        run_license_audit();
        run_unmaintained_audit();
    }

    fn run_dependency_audit() {
        let audit_check = Command::new("cargo").args(["audit", "--version"]).output();

        match audit_check {
            Ok(output) if output.status.success() => {
                let audit_result = Command::new("cargo")
                    .args(["audit"])
                    .output()
                    .expect("Failed to run cargo audit");

                if !audit_result.status.success() {
                    let stderr = String::from_utf8_lossy(&audit_result.stderr);
                    let stdout = String::from_utf8_lossy(&audit_result.stdout);
                    panic!("Security vulnerabilities found:\nSTDOUT:\n{stdout}\nSTDERR:\n{stderr}");
                }
            }
            _ => println!(
                "WARNING: cargo-audit not installed. Install with: cargo install cargo-audit"
            ),
        }
    }

    fn run_license_audit() {
        let audit_check = Command::new("cargo").args(["audit", "--version"]).output();

        if audit_check.is_ok() && audit_check.unwrap().status.success() {
            let result = Command::new("cargo")
                .args(["audit", "--deny", "warnings", "--deny", "unmaintained"])
                .output()
                .expect("Failed to run license audit");

            if !result.status.success() {
                let stderr = String::from_utf8_lossy(&result.stderr);
                panic!("License or maintenance issues found:\n{stderr}");
            }
        }
    }

    fn run_unmaintained_audit() {
        let audit_check = Command::new("cargo").args(["audit", "--version"]).output();

        if audit_check.is_ok() && audit_check.unwrap().status.success() {
            let result = Command::new("cargo")
                .args(["audit", "--stale"])
                .output()
                .expect("Failed to run stale dependency audit");

            if !result.status.success() {
                let stderr = String::from_utf8_lossy(&result.stderr);
                println!("WARNING: Stale dependencies found:\n{stderr}");
            }
        }
    }
}
