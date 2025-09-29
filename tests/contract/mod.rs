//! Contract tests module
//! 
//! These tests validate cross-crate API boundaries and MUST FAIL initially
//! as per Constitutional Principle I (Test-Driven Development).

mod test_core_issue;
mod test_core_user; 
mod test_core_project;
mod test_fs_storage;
mod test_fs_config;
mod test_fs_git;
mod test_net_sync;
mod test_net_protocol;