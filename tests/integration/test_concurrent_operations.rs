//! Concurrent operation stress tests for ODI
//!
//! Tests ODI's behavior under concurrent load to ensure data integrity
//! and proper locking mechanisms.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Barrier;
use tokio::time::timeout;
use odi_core::{Issue, IssueRepository, IssueQuery};
use odi_fs::{FileSystemStorage, FsIssueRepository};

/// Test concurrent issue creation
#[tokio::test]
async fn test_concurrent_issue_creation() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let storage = Arc::new(FileSystemStorage::new(temp_dir.path()).await.expect("Failed to create storage"));
    
    const CONCURRENT_WORKERS: usize = 10;
    const ISSUES_PER_WORKER: usize = 20;
    
    // Barrier to synchronize all workers starting at the same time
    let barrier = Arc::new(Barrier::new(CONCURRENT_WORKERS));
    
    let mut handles = Vec::new();
    
    for worker_id in 0..CONCURRENT_WORKERS {
        let storage = storage.clone();
        let barrier = barrier.clone();
        
        let handle = tokio::spawn(async move {
            let repo = FsIssueRepository::new((*storage).clone());
            
            // Wait for all workers to be ready
            barrier.wait().await;
            
            let mut created_issues = Vec::new();
            
            // Create issues concurrently
            for issue_id in 0..ISSUES_PER_WORKER {
                let issue = Issue::new(
                    format!("Concurrent test issue {} from worker {}", issue_id, worker_id),
                    format!("This issue was created by worker {} as issue {}", worker_id, issue_id),
                );
                
                match repo.create(issue.clone()).await {
                    Ok(_) => created_issues.push(issue.id),
                    Err(e) => panic!("Worker {} failed to create issue {}: {}", worker_id, issue_id, e),
                }
            }
            
            created_issues
        });
        
        handles.push(handle);
    }
    
    // Wait for all workers to complete with timeout
    let results = timeout(Duration::from_secs(30), futures::future::join_all(handles))
        .await
        .expect("Concurrent issue creation timed out");
    
    // Verify all workers succeeded
    let mut total_created = 0;
    for result in results {
        let created_issues = result.expect("Worker task failed");
        total_created += created_issues.len();
    }
    
    assert_eq!(total_created, CONCURRENT_WORKERS * ISSUES_PER_WORKER);
    
    // Verify all issues were actually stored
    let repo = FsIssueRepository::new((*storage).clone());
    let stored_issues = repo.list(IssueQuery::default()).await.expect("Failed to list issues");
    assert_eq!(stored_issues.len(), total_created);
    
    println!("✅ Successfully created {} issues with {} concurrent workers", total_created, CONCURRENT_WORKERS);
}

/// Test concurrent read/write operations
#[tokio::test]
async fn test_concurrent_read_write() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let storage = Arc::new(FileSystemStorage::new(temp_dir.path()).await.expect("Failed to create storage"));
    let repo = Arc::new(FsIssueRepository::new((*storage).clone()));
    
    // Create some initial issues
    const INITIAL_ISSUES: usize = 50;
    let mut initial_ids = Vec::new();
    
    for i in 0..INITIAL_ISSUES {
        let issue = Issue::new(
            format!("Initial issue {}", i),
            format!("Issue {} for concurrent testing", i),
        );
        initial_ids.push(issue.id.clone());
        repo.create(issue).await.expect("Failed to create initial issue");
    }
    
    const CONCURRENT_READERS: usize = 5;
    const CONCURRENT_WRITERS: usize = 3;
    const OPERATIONS_PER_WORKER: usize = 10;
    
    let barrier = Arc::new(Barrier::new(CONCURRENT_READERS + CONCURRENT_WRITERS));
    let mut handles = Vec::new();
    
    // Spawn reader workers
    for reader_id in 0..CONCURRENT_READERS {
        let repo = repo.clone();
        let barrier = barrier.clone();
        let initial_ids = initial_ids.clone();
        
        let handle = tokio::spawn(async move {
            barrier.wait().await;
            
            for _ in 0..OPERATIONS_PER_WORKER {
                // List all issues
                let issues = repo.list(IssueQuery::default()).await
                    .expect("Reader failed to list issues");
                
                // Verify we can read at least the initial issues
                assert!(issues.len() >= INITIAL_ISSUES, 
                    "Reader {} found only {} issues, expected at least {}", 
                    reader_id, issues.len(), INITIAL_ISSUES);
                
                // Sleep briefly to allow writers to work
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });
        
        handles.push(handle);
    }
    
    // Spawn writer workers
    for writer_id in 0..CONCURRENT_WRITERS {
        let repo = repo.clone();
        let barrier = barrier.clone();
        
        let handle = tokio::spawn(async move {
            barrier.wait().await;
            
            for op_id in 0..OPERATIONS_PER_WORKER {
                // Create new issues while readers are active
                let issue = Issue::new(
                    format!("Concurrent write issue {} from writer {}", op_id, writer_id),
                    format!("Written by worker {} operation {}", writer_id, op_id),
                );
                
                repo.create(issue).await
                    .expect("Writer failed to create issue");
                
                // Sleep briefly to allow readers to work
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all workers to complete
    timeout(Duration::from_secs(30), futures::future::join_all(handles))
        .await
        .expect("Concurrent read/write test timed out")
        .into_iter()
        .for_each(|result| result.expect("Worker task failed"));
    
    // Verify final state
    let final_issues = repo.list(IssueQuery::default()).await.expect("Failed to list final issues");
    let expected_total = INITIAL_ISSUES + (CONCURRENT_WRITERS * OPERATIONS_PER_WORKER);
    
    assert_eq!(final_issues.len(), expected_total, 
        "Expected {} total issues, found {}", expected_total, final_issues.len());
    
    println!("✅ Concurrent read/write test passed: {} readers, {} writers, {} final issues", 
        CONCURRENT_READERS, CONCURRENT_WRITERS, final_issues.len());
}

/// Test storage integrity under concurrent access
#[tokio::test]
async fn test_storage_integrity_under_load() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let storage = Arc::new(FileSystemStorage::new(temp_dir.path()).await.expect("Failed to create storage"));
    
    const CONCURRENT_WORKERS: usize = 8;
    const OPERATIONS_PER_WORKER: usize = 25;
    
    let barrier = Arc::new(Barrier::new(CONCURRENT_WORKERS));
    let mut handles = Vec::new();
    
    for worker_id in 0..CONCURRENT_WORKERS {
        let storage = storage.clone();
        let barrier = barrier.clone();
        
        let handle = tokio::spawn(async move {
            let repo = FsIssueRepository::new((*storage).clone());
            barrier.wait().await;
            
            let mut worker_issues = Vec::new();
            
            for op_id in 0..OPERATIONS_PER_WORKER {
                // Create issue
                let mut issue = Issue::new(
                    format!("Integrity test {} worker {}", op_id, worker_id),
                    format!("Testing storage integrity under load"),
                );
                
                // Add some variety to test different serialization paths
                if op_id % 3 == 0 {
                    issue.assignees.push(format!("user-{}", worker_id));
                }
                if op_id % 5 == 0 {
                    issue.labels.push(format!("label-{}", op_id));
                }
                
                repo.create(issue.clone()).await
                    .expect("Failed to create issue during integrity test");
                
                worker_issues.push(issue);
                
                // Periodically read back issues to verify integrity
                if op_id % 5 == 4 {
                    let retrieved_issues = repo.list(IssueQuery::default()).await
                        .expect("Failed to retrieve issues during integrity test");
                    
                    // Verify our issues are in the results
                    for created_issue in &worker_issues {
                        assert!(retrieved_issues.iter().any(|i| i.id == created_issue.id),
                            "Worker {} issue {} not found in retrieved issues", 
                            worker_id, created_issue.id);
                    }
                }
            }
            
            worker_issues
        });
        
        handles.push(handle);
    }
    
    // Wait for all workers to complete
    let results = timeout(Duration::from_secs(60), futures::future::join_all(handles))
        .await
        .expect("Storage integrity test timed out");
    
    // Collect all created issues
    let mut all_created_issues = Vec::new();
    for result in results {
        let worker_issues = result.expect("Worker task failed");
        all_created_issues.extend(worker_issues);
    }
    
    // Verify final consistency
    let repo = FsIssueRepository::new((*storage).clone());
    let final_stored_issues = repo.list(IssueQuery::default()).await
        .expect("Failed to retrieve final issues");
    
    assert_eq!(final_stored_issues.len(), all_created_issues.len(),
        "Issue count mismatch: created {}, stored {}", 
        all_created_issues.len(), final_stored_issues.len());
    
    // Verify each created issue exists in storage
    for created_issue in &all_created_issues {
        assert!(final_stored_issues.iter().any(|i| i.id == created_issue.id),
            "Created issue {} not found in final storage", created_issue.id);
    }
    
    // Verify issue data integrity
    for stored_issue in &final_stored_issues {
        if let Some(created_issue) = all_created_issues.iter().find(|i| i.id == stored_issue.id) {
            assert_eq!(stored_issue.title, created_issue.title,
                "Title mismatch for issue {}", stored_issue.id);
            assert_eq!(stored_issue.description, created_issue.description,
                "Description mismatch for issue {}", stored_issue.id);
            assert_eq!(stored_issue.assignees, created_issue.assignees,
                "Assignees mismatch for issue {}", stored_issue.id);
            assert_eq!(stored_issue.labels, created_issue.labels,
                "Labels mismatch for issue {}", stored_issue.id);
        }
    }
    
    println!("✅ Storage integrity test passed: {} workers, {} issues, all data consistent", 
        CONCURRENT_WORKERS, all_created_issues.len());
}

/// Test error handling under concurrent access
#[tokio::test]
async fn test_concurrent_error_handling() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let storage = Arc::new(FileSystemStorage::new(temp_dir.path()).await.expect("Failed to create storage"));
    
    const CONCURRENT_WORKERS: usize = 5;
    let barrier = Arc::new(Barrier::new(CONCURRENT_WORKERS));
    let mut handles = Vec::new();
    
    for worker_id in 0..CONCURRENT_WORKERS {
        let storage = storage.clone();
        let barrier = barrier.clone();
        
        let handle = tokio::spawn(async move {
            let repo = FsIssueRepository::new((*storage).clone());
            barrier.wait().await;
            
            let mut success_count = 0;
            let mut error_count = 0;
            
            for op_id in 0..10 {
                let issue = Issue::new(
                    format!("Error test {} worker {}", op_id, worker_id),
                    "Testing error handling under concurrent load".to_string(),
                );
                
                match repo.create(issue).await {
                    Ok(_) => success_count += 1,
                    Err(_) => error_count += 1,
                }
                
                // Introduce some randomness in timing
                let delay_ms = ((worker_id + op_id) % 10) as u64;
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            }
            
            (success_count, error_count)
        });
        
        handles.push(handle);
    }
    
    // Wait for all workers to complete
    let results = timeout(Duration::from_secs(30), futures::future::join_all(handles))
        .await
        .expect("Concurrent error handling test timed out");
    
    let mut total_success = 0;
    let mut total_errors = 0;
    
    for result in results {
        let (success, errors) = result.expect("Worker task failed");
        total_success += success;
        total_errors += errors;
    }
    
    println!("✅ Concurrent error handling test: {} successes, {} errors", 
        total_success, total_errors);
    
    // Most operations should succeed under normal conditions
    assert!(total_success > total_errors, 
        "Too many errors: {} successes vs {} errors", total_success, total_errors);
    
    // Verify the repository is still functional after concurrent access
    let repo = FsIssueRepository::new((*storage).clone());
    let final_issues = repo.list(IssueQuery::default()).await
        .expect("Repository not functional after concurrent access");
    
    assert_eq!(final_issues.len(), total_success,
        "Issue count mismatch: {} successes but {} stored issues", 
        total_success, final_issues.len());
}