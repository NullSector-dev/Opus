use crate::project::Project;
use anyhow::{Result, Context};
use std::{fs, path::PathBuf};

pub fn config_dir() -> PathBuf
{
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("opus");
    path
}


pub fn project_files() -> PathBuf
{
    let mut path = config_dir();
    path.push("project.json");
    path
}

pub fn load_projects() -> Result<Vec<Project>>
{
    let path = project_files();
    if !path.exists(){return Ok(vec![])}
    let data = fs::read_to_string(path).context("Path not found")?;
    Ok(serde_json::from_str(&data)?)
}

pub fn save_projects(projects: &Vec<Project>) -> Result<()>
{
    let path = project_files();
    fs::create_dir_all(config_dir()).context("Failed to Detect or Create the File")?;
    let json = serde_json::to_string_pretty(projects).context("Failed to format project into json")?;
    fs::write(path, json).context("Failed to Write to File'")?;
    Ok(())
}
