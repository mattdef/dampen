//! State management for Gravity applications

use gravity_core::UiBindable;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Wrapper for serializable runtime state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeState<T> {
    /// Serialized model data
    pub model: T,

    /// Schema version for migration
    pub version: u32,

    /// Timestamp of last save (seconds since epoch)
    pub saved_at: u64,
}

impl<T> RuntimeState<T> {
    /// Create a new runtime state from a model
    pub fn new(model: T) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            model,
            version: 1,
            saved_at: timestamp,
        }
    }
}

impl<T: Serialize> RuntimeState<T> {
    /// Serialize state to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl<T: for<'de> Deserialize<'de>> RuntimeState<T> {
    /// Deserialize state from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Result of attempting to restore state
#[derive(Debug, Clone)]
pub enum StateRestoration<T> {
    /// Full restoration successful
    Restored(T),

    /// Partial restoration with default values for new fields
    Partial {
        model: T,
        missing_fields: Vec<String>,
    },

    /// Cannot restore, using defaults
    Default { model: T, reason: String },
}

impl<T> StateRestoration<T> {
    /// Check if restoration was fully successful
    pub fn is_full(&self) -> bool {
        matches!(self, StateRestoration::Restored(_))
    }

    /// Check if restoration was partial
    pub fn is_partial(&self) -> bool {
        matches!(self, StateRestoration::Partial { .. })
    }

    /// Get the model, regardless of restoration type
    pub fn into_model(self) -> T {
        match self {
            StateRestoration::Restored(m) => m,
            StateRestoration::Partial { model, .. } => model,
            StateRestoration::Default { model, .. } => model,
        }
    }
}

/// Trait for state migration and lenient deserialization
pub trait StateMigration: Sized {
    /// Attempt to migrate from a previous version
    fn migrate_from(json: &str) -> Result<StateRestoration<Self>, serde_json::Error>
    where
        Self: DeserializeOwned + Default,
    {
        // Try direct deserialization first
        if let Ok(model) = serde_json::from_str::<Self>(json) {
            return Ok(StateRestoration::Restored(model));
        }

        // Try lenient deserialization with default for missing fields
        #[derive(Deserialize)]
        struct Lenient<T> {
            #[serde(flatten)]
            data: T,
            #[serde(default)]
            _extra: serde_json::Value,
        }

        if let Ok(lenient) = serde_json::from_str::<Lenient<Self>>(json) {
            // Check if there were extra fields that were dropped
            let parsed: serde_json::Value = serde_json::from_str(json)?;
            let actual_keys = parsed
                .as_object()
                .map(|o| o.keys().cloned().collect::<Vec<_>>())
                .unwrap_or_default();
            let expected_keys = Self::available_fields();

            let missing: Vec<String> = actual_keys
                .into_iter()
                .filter(|k| !expected_keys.contains(k))
                .collect();

            if missing.is_empty() {
                Ok(StateRestoration::Restored(lenient.data))
            } else {
                Ok(StateRestoration::Partial {
                    model: lenient.data,
                    missing_fields: missing,
                })
            }
        } else {
            // Cannot restore, use defaults
            Ok(StateRestoration::Default {
                model: Self::default(),
                reason: "Failed to deserialize state".to_string(),
            })
        }
    }

    /// Get available fields for this state type
    fn available_fields() -> Vec<String>;
}

impl<T> StateMigration for T
where
    T: UiBindable + Default + DeserializeOwned + Serialize,
{
    fn available_fields() -> Vec<String> {
        T::available_fields()
    }
}

/// Helper to save state to a file
pub fn save_state_to_file<T: Serialize>(
    state: &RuntimeState<T>,
    path: &std::path::Path,
) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(state)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Helper to load state from a file
pub fn load_state_from_file<T: for<'de> Deserialize<'de>>(
    path: &std::path::Path,
) -> Result<RuntimeState<T>, std::io::Error> {
    let json = std::fs::read_to_string(path)?;
    let state = serde_json::from_str(&json)?;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    struct TestModel {
        count: i32,
        name: String,
    }

    #[test]
    fn test_state_serialization() {
        let model = TestModel {
            count: 42,
            name: "Test".to_string(),
        };

        let state = RuntimeState::new(model.clone());
        let json = state.to_json().unwrap();

        assert!(json.contains("count"));
        assert!(json.contains("42"));
        assert!(json.contains("name"));
        assert!(json.contains("Test"));
    }

    #[test]
    fn test_state_deserialization() {
        let json = r#"{"model":{"count":42,"name":"Test"},"version":1,"saved_at":123456}"#;
        let state: RuntimeState<TestModel> = RuntimeState::from_json(json).unwrap();

        assert_eq!(state.model.count, 42);
        assert_eq!(state.model.name, "Test");
        assert_eq!(state.version, 1);
    }

    #[test]
    fn test_state_restoration_full() {
        let model = TestModel {
            count: 100,
            name: "Restored".to_string(),
        };

        let state = RuntimeState::new(model);
        let json = state.to_json().unwrap();

        let restored: RuntimeState<TestModel> = RuntimeState::from_json(&json).unwrap();
        assert_eq!(restored.model.count, 100);
        assert_eq!(restored.model.name, "Restored");
    }

    #[test]
    fn test_file_save_load() {
        use std::fs;
        use std::path::PathBuf;

        // Create a temporary directory
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_gravity_state.json");

        let original_state = RuntimeState::new(TestModel {
            count: 42,
            name: "File Test".to_string(),
        });

        // Save
        save_state_to_file(&original_state, &file_path).unwrap();

        // Load
        let loaded_state: RuntimeState<TestModel> = load_state_from_file(&file_path).unwrap();

        assert_eq!(loaded_state.model.count, original_state.model.count);
        assert_eq!(loaded_state.model.name, original_state.model.name);

        // Cleanup
        let _ = fs::remove_file(&file_path);
    }
}
