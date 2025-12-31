//! CLI commands

pub mod build;
pub mod check;
pub mod dev;
pub mod inspect;

pub use build::{BuildArgs, execute as build_execute};
pub use check::{CheckArgs, execute as check_execute};
pub use dev::{DevArgs, execute as dev_execute};
pub use inspect::{InspectArgs, execute as inspect_execute};
