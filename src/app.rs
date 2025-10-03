use crate::{project::Project, task::Task, ui, input, store::{load_projects, save_projects}};
use anyhow::{Result, Context};
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{terminal, execute, event};
use std::{io::stdout, path::Path, fs};

#[derive(PartialEq)]
pub enum Mode
{
    Projects,
    Tasks,
}

pub enum InputMode
{
    Normal,
    EditingProject(String),
    EditingTask(String),
}

pub struct App
{
    pub mode: Mode,
    pub projects: Vec<Project>,
    pub tasks: Vec<Task>,
    pub selected: usize,
    pub current_project: Option<Project>,
    pub input_mode: InputMode,
}

impl App
{
    pub fn load() -> Result<Self>
    {
        let projects = load_projects().context("Failed to load projects")?;
        Ok(Self {mode: Mode::Projects, projects, tasks: vec![], selected: 0, current_project: None, input_mode: InputMode::Normal,})
    }

    pub fn run(&mut self) -> Result<()>
    {
        let mut stdout = stdout();
        terminal::enable_raw_mode().context("Failed to enable raw mode")?;
        execute!(stdout, terminal::EnterAlternateScreen).context("Failed to enter alternate Screen")?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).context("Failed to launch a new Terminal Backend")?;

        loop
        {
            terminal.draw(|f| ui::draw(f, self))?;
            if let event::Event::Key(key) = event::read()?{ if input::key_logic(self, key)?{break;} }
        }

        terminal::disable_raw_mode().context("Failed to disable Raw Mode")?;
        execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen).context("Failed to Exit Alternate Screen")?;
        terminal.show_cursor().context("Failed to End Session Properly")?;
        Ok(())
    }


    pub fn add_project(&mut self, name: &str) -> Result<()>
    {
        if name.is_empty(){return Ok(());}

        let project = crate::project::Project::new(name);
        self.projects.push(project);
        save_projects(&self.projects).context("Failed to save Project")?;
        Ok(())
    }

    pub fn delete_project(&mut self) -> Result<()>
    {
        if let Some(project) = self.projects.get(self.selected)
        {
            let path = Path::new(&project.path);
            if path.exists(){fs::remove_file(path).context("Failed to delete the Project")?;}
            self.projects.remove(self.selected);
            if self.selected >= self.projects.len() && !self.projects.is_empty(){self.selected = self.projects.len() - 1;}
            save_projects(&self.projects).context("Failed to save Project")?;
        }
        Ok(())
    }

    pub fn next_project(&mut self)
    {
        if !self.projects.is_empty(){self.selected = (self.selected + 1) % self.projects.len();}
    }

    pub fn previous_project(&mut self)
    {
        if !self.projects.is_empty(){if self.selected == 0{self.selected = self.projects.len() - 1;} else {self.selected -= 1;}}
    }

    pub fn open_project(&mut self) -> Result<()>
    {
        if let Some(project) = self.projects.get(self.selected).cloned()
        {
            self.tasks = project.load_tasks().context("Failed to load Tasks")?;
            self.current_project = Some(project);
            self.mode = Mode::Tasks;
            self.selected = 0;
        }
        Ok(())
    }

    pub fn close_project(&mut self)
    {
        self.mode = Mode::Projects;
        self.current_project = None;
        self.tasks.clear();
        self.selected = 0;
    }

    pub fn add_task(&mut self, taskadd: &str) -> Result<()>
    {
        if let Some(project) = &self.current_project
        {
            if taskadd.is_empty(){return Ok(());}
            let id = if let Some(last) = self.tasks.last(){last.id + 1} else {1};
            self.tasks.push(Task {id, title: taskadd.to_string(), done: false,});
            project.save_tasks(&self.tasks).context("Failed to save task")?;
        }
        Ok(())
    }

    pub fn delete_task(&mut self) -> Result<()>
    {
        if let Some(project) = &self.current_project
        {
            if !self.tasks.is_empty()
                      {
                          self.tasks.remove(self.selected);
                          if self.selected >= self.tasks.len() && !self.tasks.is_empty(){self.selected = self.tasks.len() - 1;}
                          project.save_tasks(&self.tasks).context("Failed to Save Task")?;
                      }
        }
        Ok(())
    }

    pub fn next_task(&mut self)
    {
        if !self.tasks.is_empty(){self.selected = (self.selected + 1) % self.tasks.len();}
    }

    pub fn previous_task(&mut self)
    {
        if !self.tasks.is_empty(){if self.selected == 0{self.selected = self.tasks.len() - 1;} else {self.selected -= 1;}}
    }

    pub fn toggle_task(&mut self)
    {
        if let Some(task) = self.tasks.get_mut(self.selected){task.done = !task.done; if let Some(project) = &self.current_project{let _ = project.save_tasks(&self.tasks);}}
    }


}
