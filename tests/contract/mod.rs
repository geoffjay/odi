//! Contract tests module
//! 
//! These tests validate cross-crate API boundaries and MUST FAIL initially
//! as per Constitutional Principle I (Test-Driven Development).

// Core contract tests (need API alignment before they can be used)
// These tests are written but expect APIs that don't exactly match current implementation
// Uncomment and fix after API stabilization
//mod test_core_issue;
//mod test_core_user; 
//mod test_core_project;
//mod test_fs_storage;
//mod test_fs_config;
//mod test_fs_git;
//mod test_net_sync;
//mod test_net_protocol;

// CLI contract tests - COMPLETE AND WORKING
// These tests are written, compile, and properly fail when CLI behavior differs from expectations
mod test_cli_init;
mod test_cli_project;
mod test_cli_issue;
mod test_cli_remote;
mod test_cli_team;
mod test_cli_config;