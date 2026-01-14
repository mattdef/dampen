//! Shared state for inter-window communication (e.g., statistics window)

use dampen_macros::UiModel;
use serde::{Deserialize, Serialize};

/// A pending task to be added to the main window
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingTask {
    pub text: String,
    pub description: String,
    pub category: String,
    pub priority: String,
}

/// Shared state accessible across multiple windows
#[derive(Default, Clone, Debug, Serialize, Deserialize, UiModel)]
pub struct SharedState {
    /// Total number of tasks
    pub total_tasks: i64,
    /// Number of completed tasks
    pub completed_tasks: i64,
    /// Number of pending (active) tasks
    pub pending_tasks: i64,
    /// Completion percentage (0-100)
    pub completion_percentage: i64,
    /// Total tasks by category (JSON-encoded for display)
    pub category_breakdown: String,
    /// Queue of tasks to be added (consumed by main window)
    #[ui_skip]
    pub pending_task_queue: Vec<PendingTask>,
}

impl SharedState {
    /// Add a task to the pending queue
    pub fn add_pending_task(&mut self, task: PendingTask) {
        self.pending_task_queue.push(task);
    }

    /// Consume all pending tasks from the queue
    pub fn take_pending_tasks(&mut self) -> Vec<PendingTask> {
        std::mem::take(&mut self.pending_task_queue)
    }

    /// Update statistics from task data
    #[allow(dead_code)]
    pub fn update_from_tasks(&mut self, items: &[crate::ui::window::TodoItem]) {
        let total = items.len() as i64;
        let completed = items.iter().filter(|i| i.completed).count() as i64;
        let pending = total - completed;

        self.total_tasks = total;
        self.completed_tasks = completed;
        self.pending_tasks = pending;
        self.completion_percentage = if total > 0 {
            (completed * 100) / total
        } else {
            0
        };

        // Build category breakdown (simple text format for now)
        let mut categories = std::collections::HashMap::new();
        for item in items {
            *categories.entry(item.category.clone()).or_insert(0) += 1;
        }

        let breakdown: Vec<String> = categories
            .iter()
            .map(|(cat, count)| format!("{}: {}", cat, count))
            .collect();

        self.category_breakdown = breakdown.join(", ");
    }
}
