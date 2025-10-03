use crate::task::Task;
use crate::store::config_dir;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf};

#[derive(Clone, Serialize, Deserialize)]
pub struct Project
{
    pub name: String,
    pub path: String,
}

impl Project
{
    pub fn task_path(name: &str) -> PathBuf
    {
        let mut path = config_dir();
        path.push("projects");
        fs::create_dir_all(&path).ok();
        path.push(format!("{}.json", name));
        path
    }

    pub fn new(name: &str) -> Self
    {
        let path = Project::task_path(name);
        Self {name: name.to_string(), path: path.to_string_lossy().to_string(),}
    }

    pub fn load_tasks(&self) -> Result<Vec<Task>>
    {
        let path = PathBuf::from(&self.path);
        if !path.exists(){return Ok(vec![])}
        let data = fs::read_to_string(&path).unwrap_or_else(|_| "[]".to_string());
        Ok(serde_json::from_str(&data).context("Failed to load data")?)
    }

    pub fn save_tasks(&self, tasks: &Vec<Task>) -> Result<()>
    {
        let path = PathBuf::from(&self.path);
        if let Some(parent) = path.parent(){fs::create_dir_all(parent).context("Failed to create task path")?;}
        let json = serde_json::to_string_pretty(tasks).context("Failed to format task to json")?;
        fs::write(path, json).context("Failed to save task")?;
        Ok(())
    }
}
