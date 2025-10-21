//! Common test utilities

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Load a fixture script by name
pub fn load_fixture(name: &str) -> String {
    let path = fixture_path(name);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to load fixture: {:?}", path))
}

/// Get path to a fixture file
pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("sample_scripts")
        .join(name)
}

/// Performance measurement helper
pub struct PerformanceMeasurement {
    start: Instant,
    label: String,
}

impl PerformanceMeasurement {
    pub fn start(label: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            label: label.into(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn finish(self) -> Duration {
        let duration = self.elapsed();
        println!("[PERF] {}: {:?}", self.label, duration);
        duration
    }

    pub fn assert_under(&self, max_duration: Duration, message: &str) {
        let elapsed = self.elapsed();
        assert!(
            elapsed < max_duration,
            "{}: {:?} exceeds maximum {:?}",
            message,
            elapsed,
            max_duration
        );
    }
}

/// Memory usage helper (simple approximation)
pub struct MemoryTracker {
    label: String,
}

impl MemoryTracker {
    pub fn start(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }

    pub fn finish(self) {
        // In a real implementation, you'd use a memory profiling crate
        println!("[MEMORY] {}: tracking complete", self.label);
    }
}

/// Create a temporary output file
pub fn temp_output_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("luau_obfuscator_test_{}", name))
}

/// Clean up temporary files
pub fn cleanup_temp_files() {
    // Clean up any test output files
    let temp_dir = std::env::temp_dir();
    if let Ok(entries) = fs::read_dir(temp_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("luau_obfuscator_test_") {
                    let _ = fs::remove_file(entry.path());
                }
            }
        }
    }
}

/// Assert that a string contains all expected substrings
pub fn assert_contains_all(haystack: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            haystack.contains(needle),
            "Expected to find '{}' in output",
            needle
        );
    }
}

/// Assert that a string does not contain any forbidden substrings
pub fn assert_not_contains_any(haystack: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            !haystack.contains(needle),
            "Did not expect to find '{}' in output",
            needle
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_loading() {
        let content = load_fixture("simple.lua");
        assert!(content.contains("Hello, Roblox!"));
    }

    #[test]
    fn test_performance_measurement() {
        let perf = PerformanceMeasurement::start("test");
        std::thread::sleep(Duration::from_millis(10));
        let duration = perf.finish();
        assert!(duration >= Duration::from_millis(10));
    }

    #[test]
    fn test_assert_contains_all() {
        let text = "Hello World Foo Bar";
        assert_contains_all(text, &["Hello", "World", "Foo"]);
    }

    #[test]
    #[should_panic]
    fn test_assert_contains_all_failure() {
        let text = "Hello World";
        assert_contains_all(text, &["Hello", "Missing"]);
    }
}
