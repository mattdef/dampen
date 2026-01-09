//! CLI commands

pub mod build;
pub mod check;
pub mod inspect;
pub mod new;

pub use build::{execute as build_execute, BuildArgs};
pub use check::{execute as check_execute, CheckArgs};
pub use inspect::{execute as inspect_execute, InspectArgs};
pub use new::{execute as new_execute, NewArgs};
