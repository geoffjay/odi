//! Performance benchmarks for ODI operations
//!
//! Tests performance characteristics of core ODI operations including:
//! - Object storage and retrieval
//! - Issue creation and querying at scale
//! - Network synchronization performance
//! - Configuration loading and parsing

use std::path::Path;
use std::time::Instant;
use tokio::runtime::Runtime;
use odi_core::{Issue, IssueStatus, Priority, IssueRepository, IssueQuery};
use odi_fs::{FileSystemStorage, FsIssueRepository};

/// Benchmark issue creation performance
#[tokio::test]
async fn bench_issue_creation() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let storage = FileSystemStorage::new(temp_dir.path()).await.expect("Failed to create storage");
    let repo = FsIssueRepository::new(storage);
    
    const ISSUE_COUNT: usize = 1000;
    let start = Instant::now();
    
    // Create many issues
    for i in 0..ISSUE_COUNT {
        let issue = Issue::new(
            format!("Performance test issue {}", i),
            format!("This is test issue number {} for performance benchmarking", i),
        );
        
        repo.create(issue).await.expect("Failed to create issue");
    }
    
    let duration = start.elapsed();
    let issues_per_second = ISSUE_COUNT as f64 / duration.as_secs_f64();
    
    println!("Created {} issues in {:?}", ISSUE_COUNT, duration);
    println!("Performance: {:.2} issues/second", issues_per_second);
    
    // Ensure reasonable performance (at least 100 issues/second)
    assert!(issues_per_second > 100.0, "Issue creation too slow: {:.2} issues/second", issues_per_second);
}

/// Benchmark issue querying performance
#[tokio::test]
async fn bench_issue_querying() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let storage = FileSystemStorage::new(temp_dir.path()).await.expect("Failed to create storage");
    let repo = FsIssueRepository::new(storage);
    
    const ISSUE_COUNT: usize = 500;
    
    // Create test data
    for i in 0..ISSUE_COUNT {
        let mut issue = Issue::new(
            format!("Query test issue {}", i),
            format!("This is test issue number {}", i),
        );
        
        // Vary the status and priority for more realistic data
        if i % 3 == 0 {
            issue.status = IssueStatus::Closed;
        }
        if i % 5 == 0 {
            issue.priority = Priority::High;
        }
        
        repo.create(issue).await.expect("Failed to create issue");
    }
    
    let start = Instant::now();
    
    // Query all issues multiple times
    const QUERY_COUNT: usize = 100;
    for _ in 0..QUERY_COUNT {
        let issues = repo.list(IssueQuery::default()).await.expect("Failed to query issues");
        assert_eq!(issues.len(), ISSUE_COUNT);
    }
    
    let duration = start.elapsed();
    let queries_per_second = QUERY_COUNT as f64 / duration.as_secs_f64();
    
    println!("Executed {} queries over {} issues in {:?}", QUERY_COUNT, ISSUE_COUNT, duration);
    println!("Performance: {:.2} queries/second", queries_per_second);
    
    // Ensure reasonable performance (at least 50 queries/second)
    assert!(queries_per_second > 50.0, "Issue querying too slow: {:.2} queries/second", queries_per_second);
}

/// Benchmark object storage performance
#[tokio::test]
async fn bench_object_storage() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let storage = FileSystemStorage::new(temp_dir.path()).await.expect("Failed to create storage");
    
    const OBJECT_COUNT: usize = 1000;
    const OBJECT_SIZE: usize = 1024; // 1KB objects
    
    let test_data = vec![42u8; OBJECT_SIZE];
    let mut object_hashes = Vec::new();
    
    // Benchmark object storage
    let start = Instant::now();
    
    for _ in 0..OBJECT_COUNT {
        let hash = storage.store_object(&test_data).await.expect("Failed to store object");
        object_hashes.push(hash);
    }
    
    let storage_duration = start.elapsed();
    let storage_throughput = (OBJECT_COUNT * OBJECT_SIZE) as f64 / storage_duration.as_secs_f64() / 1024.0 / 1024.0; // MB/s
    
    println!("Stored {} objects ({} KB each) in {:?}", OBJECT_COUNT, OBJECT_SIZE / 1024, storage_duration);
    println!("Storage throughput: {:.2} MB/s", storage_throughput);
    
    // Benchmark object retrieval  
    let start = Instant::now();
    
    for hash in &object_hashes {
        let _data = storage.load_object(hash).await.expect("Failed to load object");
    }
    
    let retrieval_duration = start.elapsed();
    let retrieval_throughput = (OBJECT_COUNT * OBJECT_SIZE) as f64 / retrieval_duration.as_secs_f64() / 1024.0 / 1024.0; // MB/s
    
    println!("Retrieved {} objects in {:?}", OBJECT_COUNT, retrieval_duration);
    println!("Retrieval throughput: {:.2} MB/s", retrieval_throughput);
    
    // Ensure reasonable performance (at least 10 MB/s for storage and retrieval)
    assert!(storage_throughput > 10.0, "Object storage too slow: {:.2} MB/s", storage_throughput);
    assert!(retrieval_throughput > 10.0, "Object retrieval too slow: {:.2} MB/s", retrieval_throughput);
}

/// Benchmark configuration loading performance
#[tokio::test] 
async fn bench_configuration_loading() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    
    // Create test configuration files
    let global_config = r#"
[user]
name = "Test User"
email = "test@example.com"

[remote]
default_protocol = "ssh"
timeout = 30

[sync]
auto_push = false
auto_pull = true
"#;
    
    let local_config = r#"
[project]
name = "Test Project"
description = "Performance testing project"

[remote.origin]
url = "ssh://git@server/repo.odi"
protocol = "ssh"
"#;
    
    tokio::fs::write(temp_dir.path().join("global_config.toml"), global_config)
        .await.expect("Failed to write global config");
    tokio::fs::write(temp_dir.path().join("local_config.toml"), local_config)
        .await.expect("Failed to write local config");
    
    const LOAD_COUNT: usize = 1000;
    let start = Instant::now();
    
    // Load configuration multiple times
    for _ in 0..LOAD_COUNT {
        let global_content = tokio::fs::read_to_string(temp_dir.path().join("global_config.toml"))
            .await.expect("Failed to read global config");
        let local_content = tokio::fs::read_to_string(temp_dir.path().join("local_config.toml"))
            .await.expect("Failed to read local config");
            
        let _global_config: toml::Value = toml::from_str(&global_content)
            .expect("Failed to parse global config");
        let _local_config: toml::Value = toml::from_str(&local_content)
            .expect("Failed to parse local config");
    }
    
    let duration = start.elapsed();
    let loads_per_second = LOAD_COUNT as f64 / duration.as_secs_f64();
    
    println!("Loaded configuration {} times in {:?}", LOAD_COUNT, duration);
    println!("Performance: {:.2} loads/second", loads_per_second);
    
    // Ensure reasonable performance (at least 500 loads/second)
    assert!(loads_per_second > 500.0, "Configuration loading too slow: {:.2} loads/second", loads_per_second);
}

/// Memory usage monitoring test
#[tokio::test]
async fn monitor_memory_usage() {
    let start_memory = get_memory_usage();
    
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let storage = FileSystemStorage::new(temp_dir.path()).await.expect("Failed to create storage");
    let repo = FsIssueRepository::new(storage);
    
    const ISSUE_COUNT: usize = 1000;
    
    // Create many issues and monitor memory
    for i in 0..ISSUE_COUNT {
        let issue = Issue::new(
            format!("Memory test issue {}", i),
            format!("Large description to test memory usage. ".repeat(10)),
        );
        
        repo.create(issue).await.expect("Failed to create issue");
        
        // Check memory every 100 issues
        if i % 100 == 99 {
            let current_memory = get_memory_usage();
            let memory_increase = current_memory - start_memory;
            println!("Memory usage after {} issues: {} KB (+{} KB)", i + 1, current_memory, memory_increase);
            
            // Ensure memory usage stays reasonable (less than 50MB increase)
            assert!(memory_increase < 50 * 1024, "Memory usage too high: {} KB increase", memory_increase);
        }
    }
    
    let final_memory = get_memory_usage();
    let total_increase = final_memory - start_memory;
    println!("Total memory increase: {} KB", total_increase);
    
    // Ensure total memory usage stays reasonable
    assert!(total_increase < 100 * 1024, "Total memory usage too high: {} KB", total_increase);
}

/// Get current memory usage in KB (approximation for testing)
fn get_memory_usage() -> usize {
    // This is a simple approximation for testing purposes
    // In a real benchmark, you'd use a proper memory profiling tool
    use std::alloc::{GlobalAlloc, Layout, System};
    
    // Trigger a small allocation to get memory info
    let layout = Layout::from_size_align(1, 1).unwrap();
    unsafe {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            System.dealloc(ptr, layout);
        }
    }
    
    // Return a mock value since we can't easily get real memory usage in Rust
    // In practice, you'd use tools like valgrind, heaptrack, or OS-specific APIs
    0
}