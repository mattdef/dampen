//! Gravity Iced - Iced Backend Implementation
//!
//! This crate implements the Backend trait for Iced.

pub mod commands;
pub mod theme;
pub mod widgets;

/// Iced backend implementation
pub struct IcedBackend;

impl IcedBackend {
    /// Create a new Iced backend
    pub fn new() -> Self {
        Self
    }
}

impl Default for IcedBackend {
    fn default() -> Self {
        Self::new()
    }
}
