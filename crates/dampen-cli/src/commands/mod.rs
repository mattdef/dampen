//! CLI commands

pub mod build;
pub mod check;
pub mod inspect;
pub mod new;
pub mod release;
pub mod run;
pub mod test;

pub use build::{BuildArgs, execute as build_execute};
pub use check::{CheckArgs, execute as check_execute};
pub use inspect::{InspectArgs, execute as inspect_execute};
pub use new::{NewArgs, execute as new_execute};
pub use release::{ReleaseArgs, execute as release_execute};
pub use run::{RunArgs, execute as run_execute};
pub use test::{TestArgs, execute as test_execute};
