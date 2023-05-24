use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: TaskStatus,
}

impl Task {
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
}

pub struct TaskBuilder {
    task: Task,
}

impl TaskBuilder {
    pub fn new() -> TaskBuilder {
        TaskBuilder {
            task: Task {
                title: None,
                description: None,
                status: TaskStatus::NotStarted,
            },
        }
    }

    pub fn with_title(mut self, title: String) -> TaskBuilder {
        self.task.title = Some(title);
        self
    }

    pub fn with_description(mut self, description: String) -> TaskBuilder {
        self.task.description = Some(description);
        self
    }

    pub fn with_status(mut self, status: TaskStatus) -> TaskBuilder {
        self.task.status = status;
        self
    }

    pub fn build(self) -> Task {
        self.task
    }
}
