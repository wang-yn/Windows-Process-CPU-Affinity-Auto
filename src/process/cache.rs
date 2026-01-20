use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

#[derive(Debug)]
#[allow(dead_code)]
struct ProcessEntry {
    pid: u32,
    name: String,
    last_seen: Instant,
    processed: bool,
}

pub struct ProcessCache {
    /// Cache of PIDs that have been seen
    processes: HashMap<u32, ProcessEntry>,
    /// PIDs that have been successfully processed
    processed_pids: HashSet<u32>,
    /// Maximum age before a process entry is considered stale
    max_age: Duration,
}

impl ProcessCache {
    pub fn new(cleanup_interval_secs: u64) -> Self {
        Self {
            processes: HashMap::new(),
            processed_pids: HashSet::new(),
            max_age: Duration::from_secs(cleanup_interval_secs),
        }
    }

    /// Check if a process has already been seen
    pub fn is_new_process(&self, pid: u32) -> bool {
        !self.processes.contains_key(&pid)
    }

    /// Check if a process has been successfully processed
    pub fn is_processed(&self, pid: u32) -> bool {
        self.processed_pids.contains(&pid)
    }

    /// Mark a process as seen
    pub fn mark_seen(&mut self, pid: u32, name: String) {
        self.processes.insert(
            pid,
            ProcessEntry {
                pid,
                name,
                last_seen: Instant::now(),
                processed: false,
            },
        );
    }

    /// Mark a process as successfully processed
    pub fn mark_processed(&mut self, pid: u32) {
        self.processed_pids.insert(pid);
        if let Some(entry) = self.processes.get_mut(&pid) {
            entry.processed = true;
            entry.last_seen = Instant::now();
        }
    }

    /// Clean up stale process entries
    pub fn cleanup(&mut self) -> usize {
        let now = Instant::now();
        let initial_count = self.processes.len();

        // Remove stale entries
        self.processes.retain(|pid, entry| {
            let is_fresh = now.duration_since(entry.last_seen) < self.max_age;
            if !is_fresh {
                // Also remove from processed set
                self.processed_pids.remove(pid);
            }
            is_fresh
        });

        let removed = initial_count - self.processes.len();
        if removed > 0 {
            log::debug!("Cleaned up {} stale process entries from cache", removed);
        }

        removed
    }

    /// Get the number of processes in cache
    pub fn len(&self) -> usize {
        self.processes.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.processes.is_empty()
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            total_entries: self.processes.len(),
            processed_count: self.processed_pids.len(),
            unprocessed_count: self.processes.len() - self.processed_pids.len(),
        }
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub total_entries: usize,
    pub processed_count: usize,
    pub unprocessed_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_new_process_detection() {
        let mut cache = ProcessCache::new(300);

        assert!(cache.is_new_process(1234));
        cache.mark_seen(1234, "test.exe".to_string());
        assert!(!cache.is_new_process(1234));
    }

    #[test]
    fn test_process_marking() {
        let mut cache = ProcessCache::new(300);

        cache.mark_seen(1234, "test.exe".to_string());
        assert!(!cache.is_processed(1234));

        cache.mark_processed(1234);
        assert!(cache.is_processed(1234));
    }

    #[test]
    fn test_cache_cleanup() {
        let mut cache = ProcessCache::new(1); // 1 second max age

        cache.mark_seen(1234, "test.exe".to_string());
        assert_eq!(cache.len(), 1);

        thread::sleep(Duration::from_secs(2));
        cache.cleanup();

        assert_eq!(cache.len(), 0);
        assert!(!cache.is_processed(1234));
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = ProcessCache::new(300);

        cache.mark_seen(1234, "test1.exe".to_string());
        cache.mark_seen(5678, "test2.exe".to_string());
        cache.mark_processed(1234);

        let stats = cache.stats();
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.processed_count, 1);
        assert_eq!(stats.unprocessed_count, 1);
    }
}
