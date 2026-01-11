//! CLI commands

pub mod build;
pub mod check;
pub mod inspect;
pub mod new;
pub mod run;

pub use build::{BuildArgs, execute as build_execute};
pub use check::{CheckArgs, execute as check_execute};
pub use inspect::{InspectArgs, execute as inspect_execute};
pub use new::{NewArgs, execute as new_execute};
pub use run::{RunArgs, execute as run_execute};
