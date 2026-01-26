//! Add command for scaffolding new UI windows.
//!
//! This module provides the `dampen add --ui <window_name>` command that generates
//! UI window files (`.rs` and `.dampen`) based on templates.
//!
//! # Overview
//!
//! The `add` command scaffolds new UI windows for Dampen applications by:
//! - Generating a Rust module with model, handlers, and AppState
//! - Creating a corresponding `.dampen` XML file with basic UI layout
//! - Validating window names and output paths
//! - Preventing accidental file overwrites
//!
//! # Usage
//!
//! ## Basic Usage
//!
//! Create a new window in the default location (`src/ui/`):
//!
//! ```bash
//! dampen add --ui settings
//! ```
//!
//! This generates:
//! - `src/ui/settings.rs` - Rust module with Model and handlers
//! - `src/ui/settings.dampen` - XML UI definition
//!
//! ## Custom Output Directory
//!
//! Specify a custom output directory with `--path`:
//!
//! ```bash
//! dampen add --ui order_form --path "src/ui/orders"
//! ```
//!
//! This generates files in `src/ui/orders/`:
//! - `src/ui/orders/order_form.rs`
//! - `src/ui/orders/order_form.dampen`
//!
//! ## Window Name Conventions
//!
//! Window names are automatically converted to proper case:
//! - Input: `UserProfile` → Files: `user_profile.rs`, `user_profile.dampen`
//! - Input: `settings` → Files: `settings.rs`, `settings.dampen`
//!
//! # Generated Code Structure
//!
//! The generated Rust module includes:
//! - `Model` struct with `#[derive(UiModel)]` for data binding
//! - `create_app_state()` function that returns configured `AppState<Model>`
//! - `create_handler_registry()` with sample event handlers
//! - Auto-loading via `#[dampen_ui]` macro
//!
//! The generated XML includes:
//! - Basic column layout with text and button widgets
//! - Data binding example (`{message}`)
//! - Event handler hookup (`on_click="on_action"`)
//!
//! # After Generation
//!
//! 1. Add the module to `src/ui/mod.rs`:
//!    ```rust,ignore
//!    pub mod settings;
//!    ```
//!
//! 2. Validate the XML:
//!    ```bash
//!    dampen check
//!    ```
//!
//! 3. Use in your application:
//!    ```rust,ignore
//!    use ui::settings;
//!    let state = settings::create_app_state();
//!    ```
//!
//! # Error Handling
//!
//! The command validates:
//! - Project context (must be a Dampen project with `dampen-core` dependency)
//! - Window name (must be valid Rust identifier, not a reserved keyword)
//! - Output path (must be relative, within project bounds)
//! - File conflicts (prevents overwriting existing files)
//!
//! # Examples
//!
//! ```bash
//! # Create a settings window
//! dampen add --ui settings
//!
//! # Create an admin dashboard in a subdirectory
//! dampen add --ui dashboard --path "src/ui/admin"
//!
//! # Create an order form
//! dampen add --ui OrderForm
//! # → Generates: order_form.rs, order_form.dampen
//! ```

use clap::Args;

pub mod errors;
pub mod generation;
pub mod integration;
pub mod templates;
pub mod validation;
pub mod view_switching;

// Export error types (Phase 2 complete)
pub use errors::{GenerationError, PathError, ProjectError, ValidationError};

// Export template types (Phase 2 complete)
pub use templates::{TemplateKind, WindowNameVariants, WindowTemplate};

// Export validation types (Phase 3-4 complete)
pub use validation::{ProjectInfo, TargetPath, WindowName};

// Export generation types (Phase 5)
pub use generation::{GeneratedFiles, generate_window_files};

// Types will be exported as they're implemented in later phases
// pub use validation::{TargetPath};

/// Arguments for the `dampen add` command.
///
/// # Examples
///
/// ```bash
/// # Add a window in default location (src/ui/)
/// dampen add --ui settings
///
/// # Add a window in custom location
/// dampen add --ui admin_panel --path "src/ui/admin"
/// ```
///
/// # Fields
///
/// - `ui`: Window name (converted to snake_case for filenames)
/// - `path`: Custom output directory (relative to project root)
#[derive(Debug, Args)]
pub struct AddArgs {
    /// Add a new UI window
    ///
    /// The window name will be converted to snake_case for filenames.
    ///
    /// Examples:
    ///   settings       → settings.rs, settings.dampen
    ///   UserProfile    → user_profile.rs, user_profile.dampen
    ///   admin-panel    → admin_panel.rs, admin_panel.dampen
    #[arg(long)]
    pub ui: Option<String>,

    /// Custom output directory path (relative to project root)
    ///
    /// If not provided, defaults to "src/ui/"
    ///
    /// Examples:
    ///   --path "src/ui/admin"      → Files in src/ui/admin/
    ///   --path "ui/orders"         → Files in ui/orders/
    ///
    /// Security:
    ///   - Must be relative (absolute paths rejected)
    ///   - Must be within project (cannot escape via ..)
    #[arg(long)]
    pub path: Option<String>,

    /// Disable automatic integration (do not update mod.rs)
    ///
    /// By default, the command automatically adds `pub mod <window_name>;`
    /// to the appropriate mod.rs file. Use this flag to disable automatic
    /// integration and handle module registration manually.
    ///
    /// Example:
    ///   dampen add --ui settings --no-integrate
    #[arg(long)]
    pub no_integrate: bool,
}

/// Execute the add command.
///
/// This generates UI window files based on validated inputs.
///
/// # Process
///
/// 1. **Detect project**: Validates this is a Dampen project
/// 2. **Validate name**: Checks window name is valid identifier
/// 3. **Resolve path**: Determines output directory (default or custom)
/// 4. **Generate files**: Creates .rs and .dampen files from templates
/// 5. **Report success**: Shows file paths and next steps
///
/// # Errors
///
/// Returns `Err(String)` if:
/// - Not in a Dampen project (no `dampen-core` in Cargo.toml)
/// - Window name is invalid (empty, starts with number, reserved keyword)
/// - Output path is invalid (absolute, escapes project)
/// - Files already exist (prevents overwriting)
/// - I/O errors occur during file creation
///
/// # Examples
///
/// ```no_run
/// use dampen_cli::commands::add::{AddArgs, execute};
///
/// let args = AddArgs {
///     ui: Some("settings".to_string()),
///     path: None,
///     no_integrate: false,
/// };
///
/// match execute(&args) {
///     Ok(()) => println!("Window created successfully"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn execute(args: &AddArgs) -> Result<(), String> {
    // T073: Detect and validate project
    let project_info = ProjectInfo::detect().map_err(|e| e.to_string())?;

    if !project_info.is_dampen {
        return Err(
            "Error: Not a Dampen project (dampen-core dependency not found)\nhelp: Add dampen-core to your Cargo.toml, or run 'dampen new' to create a new project"
                .to_string(),
        );
    }

    // T074: Validate window name
    let window_name_str = args
        .ui
        .as_ref()
        .ok_or_else(|| "Error: Missing window name\nhelp: Use --ui <name>".to_string())?;

    let window_name = WindowName::new(window_name_str).map_err(|e| e.to_string())?;

    // T075: Resolve target path with validation (Phase 6)
    let target_path =
        TargetPath::resolve(&project_info.root, args.path.as_deref()).map_err(|e| e.to_string())?;

    // T076: Generate files
    let enable_integration = !args.no_integrate;
    let generated = generate_window_files(&target_path, &window_name, enable_integration)
        .map_err(|e| e.to_string())?;

    // T077: Print success message
    println!("{}", generated.success_message());

    // T079: Return success
    Ok(())
}
