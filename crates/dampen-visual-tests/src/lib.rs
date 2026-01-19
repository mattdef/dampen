//! Visual regression testing infrastructure for Dampen UI framework.
//!
//! This crate provides tools to verify pixel-perfect parity between Interpreted
//! and Codegen modes by rendering widgets offscreen and comparing the output.

pub mod compare;
pub mod renderer;

use std::path::Path;

/// A visual test case that compares rendered output.
#[derive(Debug, Clone)]
pub struct VisualTestCase {
    /// Name of the test case
    pub name: String,
    /// Dampen XML content to render
    pub dampen_xml: String,
    /// Maximum allowed difference (0.0 = exact match, 1.0 = completely different)
    pub tolerance: f32,
}

impl VisualTestCase {
    /// Creates a new visual test case.
    pub fn new(name: impl Into<String>, dampen_xml: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            dampen_xml: dampen_xml.into(),
            tolerance: 0.01, // 1% default tolerance
        }
    }

    /// Sets the tolerance for image comparison.
    pub fn with_tolerance(mut self, tolerance: f32) -> Self {
        self.tolerance = tolerance;
        self
    }
}

/// Result of a visual comparison test.
#[derive(Debug)]
pub struct VisualTestResult {
    /// Name of the test
    pub name: String,
    /// Whether the test passed
    pub passed: bool,
    /// Actual difference percentage (0.0 to 1.0)
    pub difference: f32,
    /// Path to the baseline image
    pub baseline_path: Option<String>,
    /// Path to the actual rendered image
    pub actual_path: Option<String>,
    /// Path to the diff image (if generated)
    pub diff_path: Option<String>,
}

impl VisualTestResult {
    /// Creates a passing test result.
    pub fn pass(name: impl Into<String>, difference: f32) -> Self {
        Self {
            name: name.into(),
            passed: true,
            difference,
            baseline_path: None,
            actual_path: None,
            diff_path: None,
        }
    }

    /// Creates a failing test result.
    pub fn fail(name: impl Into<String>, difference: f32) -> Self {
        Self {
            name: name.into(),
            passed: false,
            difference,
            baseline_path: None,
            actual_path: None,
            diff_path: None,
        }
    }

    /// Adds paths for generated images.
    pub fn with_paths(
        mut self,
        baseline: impl AsRef<Path>,
        actual: impl AsRef<Path>,
        diff: Option<impl AsRef<Path>>,
    ) -> Self {
        self.baseline_path = Some(baseline.as_ref().display().to_string());
        self.actual_path = Some(actual.as_ref().display().to_string());
        self.diff_path = diff.map(|p| p.as_ref().display().to_string());
        self
    }
}
