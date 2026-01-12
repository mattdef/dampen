//! Research: Glob Pattern Matching for Proc-Macro File Exclusion
//!
//! This test file compares three approaches for implementing glob pattern matching
//! in the #[dampen_app] macro's `exclude` parameter.
//!
//! Run with: cargo test -p dampen-macros glob_pattern_research -- --nocapture

#[cfg(test)]
mod glob_pattern_research {
    use std::path::{Path, PathBuf};
    use std::time::Instant;

    // =============================================================================
    // APPROACH 1: Using `glob` crate (0.3.3)
    // =============================================================================

    /// Approach 1: Simple glob pattern matching using the `glob` crate
    ///
    /// Pros:
    /// - Official rust-lang crate (maintained by Rust project)
    /// - Simple API with Pattern::matches
    /// - Works at compile time (no runtime overhead)
    /// - Minimal dependencies
    /// - 1.63+ MSRV (compatible with our 1.85+)
    ///
    /// Cons:
    /// - Limited to filesystem globbing (designed for glob() function)
    /// - Pattern must be compiled for each match
    /// - No glob set optimization for multiple patterns
    mod approach_glob_crate {
        use super::*;

        pub fn matches_any_pattern(path: &Path, patterns: &[&str]) -> bool {
            // Note: glob::Pattern is available at compile time
            // We'd compile patterns once in the proc-macro and reuse them
            patterns.iter().any(|pattern| {
                // In real implementation, patterns would be pre-compiled
                if let Ok(glob_pattern) = glob::Pattern::new(pattern) {
                    glob_pattern.matches_path(path)
                } else {
                    false
                }
            })
        }

        pub fn compile_time_overhead(patterns: &[&str]) -> std::time::Duration {
            let start = Instant::now();
            let _compiled: Vec<_> = patterns
                .iter()
                .filter_map(|p| glob::Pattern::new(p).ok())
                .collect();
            start.elapsed()
        }
    }

    // =============================================================================
    // APPROACH 2: Using `globset` crate (0.4.18)
    // =============================================================================

    /// Approach 2: Advanced glob matching using `globset` (from ripgrep)
    ///
    /// Pros:
    /// - Optimized for matching many patterns at once (GlobSet)
    /// - Rich feature set (case sensitivity, literal separators, etc.)
    /// - Compile patterns once, match many paths efficiently
    /// - Production-proven (used by ripgrep)
    /// - Good error messages for invalid patterns
    ///
    /// Cons:
    /// - More dependencies (aho-corasick, regex-automata, etc.)
    /// - Slightly more complex API
    /// - May be overkill for simple use cases
    mod approach_globset_crate {
        use super::*;

        pub fn matches_any_pattern(path: &Path, patterns: &[&str]) -> bool {
            // Build a GlobSet once (would be done at macro expansion time)
            let mut builder = globset::GlobSetBuilder::new();
            for pattern in patterns {
                if let Ok(glob) = globset::Glob::new(pattern) {
                    builder.add(glob);
                }
            }

            if let Ok(set) = builder.build() {
                set.is_match(path)
            } else {
                false
            }
        }

        pub fn compile_time_overhead(patterns: &[&str]) -> std::time::Duration {
            let start = Instant::now();
            let mut builder = globset::GlobSetBuilder::new();
            for pattern in patterns {
                if let Ok(glob) = globset::Glob::new(pattern) {
                    builder.add(glob);
                }
            }
            let _ = builder.build();
            start.elapsed()
        }

        /// Example of advanced configuration
        pub fn with_options(path: &Path, patterns: &[&str]) -> bool {
            let mut builder = globset::GlobSetBuilder::new();
            for pattern in patterns {
                if let Ok(glob) = globset::GlobBuilder::new(pattern)
                    .case_insensitive(false)
                    .literal_separator(true) // * doesn't match /
                    .build()
                {
                    builder.add(glob);
                }
            }

            if let Ok(set) = builder.build() {
                set.is_match(path)
            } else {
                false
            }
        }
    }

    // =============================================================================
    // APPROACH 3: Hand-rolled simple matching
    // =============================================================================

    /// Approach 3: Custom implementation supporting only basic wildcards
    ///
    /// Pros:
    /// - Zero dependencies
    /// - Minimal compile-time overhead
    /// - Full control over behavior
    /// - Can be optimized for our specific use case
    ///
    /// Cons:
    /// - Must implement and test pattern matching ourselves
    /// - Limited feature set (only *, ?, no character classes)
    /// - May have edge cases we haven't considered
    /// - Reinventing the wheel
    mod approach_hand_rolled {
        use super::*;

        /// Simple pattern matcher supporting * and ? wildcards
        pub fn matches_pattern(path: &Path, pattern: &str) -> bool {
            let path_str = path.to_string_lossy();
            match_glob_simple(&path_str, pattern)
        }

        /// Match a simple glob pattern with * and ? support
        fn match_glob_simple(text: &str, pattern: &str) -> bool {
            let text_chars: Vec<char> = text.chars().collect();
            let pattern_chars: Vec<char> = pattern.chars().collect();
            match_recursive(&text_chars, &pattern_chars, 0, 0)
        }

        fn match_recursive(text: &[char], pattern: &[char], ti: usize, pi: usize) -> bool {
            // Both exhausted = match
            if ti == text.len() && pi == pattern.len() {
                return true;
            }

            // Pattern exhausted but text remains = no match
            if pi == pattern.len() {
                return false;
            }

            match pattern[pi] {
                '*' => {
                    // Try matching zero characters
                    if match_recursive(text, pattern, ti, pi + 1) {
                        return true;
                    }
                    // Try matching one or more characters
                    if ti < text.len() && match_recursive(text, pattern, ti + 1, pi) {
                        return true;
                    }
                    false
                }
                '?' => {
                    // Match any single character
                    if ti < text.len() {
                        match_recursive(text, pattern, ti + 1, pi + 1)
                    } else {
                        false
                    }
                }
                c => {
                    // Literal character match
                    if ti < text.len() && text[ti] == c {
                        match_recursive(text, pattern, ti + 1, pi + 1)
                    } else {
                        false
                    }
                }
            }
        }

        pub fn matches_any_pattern(path: &Path, patterns: &[&str]) -> bool {
            patterns
                .iter()
                .any(|pattern| matches_pattern(path, pattern))
        }

        pub fn compile_time_overhead(_patterns: &[&str]) -> std::time::Duration {
            // No compilation needed - patterns are used directly
            std::time::Duration::from_nanos(0)
        }
    }

    // =============================================================================
    // COMPARATIVE TESTS
    // =============================================================================

    #[test]
    fn test_exact_match() {
        let path = Path::new("debug_view.dampen");
        let patterns = &["debug_view.dampen"];

        assert!(approach_glob_crate::matches_any_pattern(path, patterns));
        assert!(approach_globset_crate::matches_any_pattern(path, patterns));
        assert!(approach_hand_rolled::matches_any_pattern(path, patterns));
    }

    #[test]
    fn test_prefix_wildcard() {
        let path = Path::new("experimental/feature.dampen");
        let patterns = &["experimental/*"];

        assert!(approach_glob_crate::matches_any_pattern(path, patterns));
        assert!(approach_globset_crate::matches_any_pattern(path, patterns));
        // Note: hand-rolled doesn't distinguish / from other chars
        assert!(approach_hand_rolled::matches_any_pattern(path, patterns));
    }

    #[test]
    fn test_prefix_match_with_basename() {
        let path = Path::new("experimental/nested/feature.dampen");
        let patterns = &["experimental/*"];

        // glob crate: * DOES match / by default (requires literal_separator option to prevent)
        assert!(approach_glob_crate::matches_any_pattern(path, patterns));

        // globset: with literal_separator=true, * doesn't match /
        assert!(!approach_globset_crate::with_options(path, patterns));

        // globset: default behavior (literal_separator=false), * matches /
        assert!(approach_globset_crate::matches_any_pattern(path, patterns));

        // hand-rolled: * matches everything
        assert!(approach_hand_rolled::matches_any_pattern(path, patterns));
    }

    #[test]
    fn test_recursive_wildcard() {
        let path = Path::new("experimental/nested/deep/feature.dampen");
        let patterns = &["experimental/**/*.dampen"];

        assert!(approach_glob_crate::matches_any_pattern(path, patterns));
        assert!(approach_globset_crate::matches_any_pattern(path, patterns));
        // hand-rolled doesn't support ** (treats it as two consecutive wildcards)
        // which actually matches the same way as a single *
        assert!(approach_hand_rolled::matches_any_pattern(path, patterns));
    }

    #[test]
    fn test_wildcard_filename() {
        let path = Path::new("test_widget.dampen");
        let patterns = &["test_*.dampen"];

        assert!(approach_glob_crate::matches_any_pattern(path, patterns));
        assert!(approach_globset_crate::matches_any_pattern(path, patterns));
        assert!(approach_hand_rolled::matches_any_pattern(path, patterns));
    }

    #[test]
    fn test_multiple_patterns() {
        let paths = vec![
            PathBuf::from("debug_view.dampen"),
            PathBuf::from("experimental/feature.dampen"),
            PathBuf::from("test_widget.dampen"),
        ];
        let patterns = &["debug_view.dampen", "experimental/*", "test_*.dampen"];

        for path in &paths {
            assert!(approach_glob_crate::matches_any_pattern(path, patterns));
            assert!(approach_globset_crate::matches_any_pattern(path, patterns));
            assert!(approach_hand_rolled::matches_any_pattern(path, patterns));
        }

        // Negative test
        let non_matching = Path::new("src/app.dampen");
        assert!(!approach_glob_crate::matches_any_pattern(
            non_matching,
            patterns
        ));
        assert!(!approach_globset_crate::matches_any_pattern(
            non_matching,
            patterns
        ));
        assert!(!approach_hand_rolled::matches_any_pattern(
            non_matching,
            patterns
        ));
    }

    #[test]
    fn test_performance_compilation() {
        let patterns = &[
            "debug_*.dampen",
            "experimental/*",
            "test_*.dampen",
            "tmp/**/*.dampen",
            "draft_*.dampen",
        ];

        println!("\n=== Compilation Time Comparison ===");

        let duration_glob = approach_glob_crate::compile_time_overhead(patterns);
        println!("glob crate:      {:?}", duration_glob);

        let duration_globset = approach_globset_crate::compile_time_overhead(patterns);
        println!("globset crate:   {:?}", duration_globset);

        let duration_handrolled = approach_hand_rolled::compile_time_overhead(patterns);
        println!("hand-rolled:     {:?}", duration_handrolled);

        // glob should be very fast (compiling patterns is simple)
        assert!(duration_glob < std::time::Duration::from_millis(1));

        // globset builds a more complex automaton, so it may take longer (up to 5ms is acceptable)
        // Note: First run may include lazy initialization overhead
        assert!(
            duration_globset < std::time::Duration::from_millis(5),
            "globset compilation took {:?}, expected < 5ms",
            duration_globset
        );

        // hand-rolled has no compilation overhead
        assert_eq!(duration_handrolled, std::time::Duration::from_nanos(0));
    }

    #[test]
    fn test_edge_cases() {
        // Empty pattern
        let path = Path::new("app.dampen");
        assert!(!approach_glob_crate::matches_any_pattern(path, &[""]));

        // Pattern with no wildcards
        assert!(approach_glob_crate::matches_any_pattern(
            path,
            &["app.dampen"]
        ));

        // Pattern matching everything
        assert!(approach_glob_crate::matches_any_pattern(
            path,
            &["*.dampen"]
        ));
        assert!(approach_glob_crate::matches_any_pattern(path, &["*"]));
    }

    #[test]
    fn test_case_sensitivity() {
        let path = Path::new("Debug_View.dampen");
        let pattern = &["debug_*.dampen"];

        // Default is case-sensitive on Unix, case-insensitive on Windows
        #[cfg(unix)]
        {
            assert!(!approach_glob_crate::matches_any_pattern(path, pattern));
            assert!(!approach_globset_crate::matches_any_pattern(path, pattern));
        }

        #[cfg(windows)]
        {
            // Windows is typically case-insensitive
            assert!(approach_glob_crate::matches_any_pattern(path, pattern));
        }
    }

    // =============================================================================
    // EXAMPLE INTEGRATION FOR PROC-MACRO
    // =============================================================================

    /// Example of how this would be used in the #[dampen_app] macro
    #[test]
    fn example_proc_macro_usage() {
        // Simulated macro attribute parsing
        let exclude_patterns = vec!["debug_view.dampen", "experimental/*", "test_*.dampen"];

        // Simulated discovered files
        let discovered_files = vec![
            PathBuf::from("src/ui/app.dampen"),
            PathBuf::from("src/ui/window.dampen"),
            PathBuf::from("src/ui/debug_view.dampen"), // excluded
            PathBuf::from("src/ui/experimental/new.dampen"), // excluded
            PathBuf::from("src/ui/test_widget.dampen"), // excluded
        ];

        // Filter using globset (recommended approach)
        let mut builder = globset::GlobSetBuilder::new();
        for pattern in &exclude_patterns {
            if let Ok(glob) = globset::Glob::new(pattern) {
                builder.add(glob);
            }
        }
        let exclusion_set = builder.build().unwrap();

        let included_files: Vec<_> = discovered_files
            .iter()
            .filter(|path| {
                // Extract relative path from src/ui/
                let rel_path = path.strip_prefix("src/ui/").unwrap();
                !exclusion_set.is_match(rel_path)
            })
            .collect();

        println!("\n=== Filtered Files ===");
        for file in &included_files {
            println!("  {}", file.display());
        }

        assert_eq!(included_files.len(), 2);
        assert!(included_files.contains(&&PathBuf::from("src/ui/app.dampen")));
        assert!(included_files.contains(&&PathBuf::from("src/ui/window.dampen")));
    }

    #[test]
    fn test_error_handling() {
        // Invalid pattern for glob crate
        let invalid_pattern = "[invalid";

        let result = glob::Pattern::new(invalid_pattern);
        assert!(result.is_err());
        println!("\nglob error: {:?}", result.unwrap_err());

        let result = globset::Glob::new(invalid_pattern);
        assert!(result.is_err());
        println!("globset error: {:?}", result.unwrap_err());
    }
}
