/// Common test utilities for integration tests
///
/// Check if integration tests should run
/// Integration tests only run when INTEGRATION_TESTS=1 is set
/// This flag enables integration tests that require tmux to be available
pub fn should_run_integration_tests() -> bool {
    std::env::var("INTEGRATION_TESTS").is_ok()
}

/// Skip test if not in proper environment
#[macro_export]
macro_rules! skip_if_not_integration_env {
    () => {
        if !$crate::common::should_run_integration_tests() {
            eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 or run outside CI");
            return;
        }
    };
}
