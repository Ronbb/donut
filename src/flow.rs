use crate::task::Task;

pub struct Flow {
    pub name: String,
    pub source_task: Task,
    pub target_task: Task,
    pub condition_script: Option<String>,
}
