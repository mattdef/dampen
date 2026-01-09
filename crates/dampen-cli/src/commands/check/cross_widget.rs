// Cross-widget validation for radio button groups
use crate::commands::check::errors::CheckError;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RadioButton {
    pub value: String,
    pub file: PathBuf,
    pub line: u32,
    pub col: u32,
    pub handler: Option<String>,
}

#[derive(Debug)]
pub struct RadioGroup {
    pub id: String,
    pub buttons: Vec<RadioButton>,
}

impl RadioGroup {
    pub fn new(id: String) -> Self {
        Self {
            id,
            buttons: Vec::new(),
        }
    }

    pub fn add_button(&mut self, button: RadioButton) {
        self.buttons.push(button);
    }

    /// Validate the radio group for duplicate values and inconsistent handlers
    pub fn validate(&self) -> Vec<CheckError> {
        let mut errors = Vec::new();

        // Check for duplicate values
        let mut seen_values: HashMap<String, &RadioButton> = HashMap::new();
        for button in &self.buttons {
            if let Some(first) = seen_values.get(&button.value) {
                // Found duplicate
                errors.push(CheckError::DuplicateRadioValue {
                    value: button.value.clone(),
                    group: self.id.clone(),
                    file: button.file.clone(),
                    line: button.line,
                    col: button.col,
                    first_file: first.file.clone(),
                    first_line: first.line,
                    first_col: first.col,
                });
            } else {
                seen_values.insert(button.value.clone(), button);
            }
        }

        // Check for inconsistent handlers
        if self.buttons.len() > 1 {
            let first_handler = &self.buttons[0].handler;
            let mut handler_list = Vec::new();
            let mut has_inconsistency = false;

            for button in &self.buttons {
                if let Some(ref h) = button.handler {
                    if !handler_list.contains(h) {
                        handler_list.push(h.clone());
                    }
                }
                if &button.handler != first_handler {
                    has_inconsistency = true;
                }
            }

            if has_inconsistency {
                // Report from the second button's location
                let button = &self.buttons[1];
                errors.push(CheckError::InconsistentRadioHandlers {
                    group: self.id.clone(),
                    file: button.file.clone(),
                    line: button.line,
                    col: button.col,
                    handlers: handler_list.join(", "),
                });
            }
        }

        errors
    }
}

#[derive(Debug, Default)]
pub struct RadioGroupValidator {
    groups: HashMap<String, RadioGroup>,
}

impl RadioGroupValidator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a radio button to the validator
    pub fn add_radio(
        &mut self,
        group_id: &str,
        value: &str,
        file: &str,
        line: u32,
        col: u32,
        handler: Option<String>,
    ) {
        let button = RadioButton {
            value: value.to_string(),
            file: PathBuf::from(file),
            line,
            col,
            handler,
        };

        self.groups
            .entry(group_id.to_string())
            .or_insert_with(|| RadioGroup::new(group_id.to_string()))
            .add_button(button);
    }

    /// Validate all radio groups
    pub fn validate(&self) -> Vec<CheckError> {
        let mut errors = Vec::new();
        for group in self.groups.values() {
            errors.extend(group.validate());
        }
        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_group_with_unique_values() {
        let mut group = RadioGroup::new("test".to_string());
        group.add_button(RadioButton {
            value: "opt1".to_string(),
            file: PathBuf::from("test.dampen"),
            line: 10,
            col: 5,
            handler: Some("handler".to_string()),
        });
        group.add_button(RadioButton {
            value: "opt2".to_string(),
            file: PathBuf::from("test.dampen"),
            line: 15,
            col: 5,
            handler: Some("handler".to_string()),
        });

        let errors = group.validate();
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_radio_group_with_duplicate_values() {
        let mut group = RadioGroup::new("test".to_string());
        group.add_button(RadioButton {
            value: "opt1".to_string(),
            file: PathBuf::from("test.dampen"),
            line: 10,
            col: 5,
            handler: Some("handler".to_string()),
        });
        group.add_button(RadioButton {
            value: "opt1".to_string(),
            file: PathBuf::from("test.dampen"),
            line: 15,
            col: 5,
            handler: Some("handler".to_string()),
        });

        let errors = group.validate();
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            CheckError::DuplicateRadioValue { value, .. } => {
                assert_eq!(value, "opt1");
            }
            _ => panic!("Expected DuplicateRadioValue error"),
        }
    }
}
