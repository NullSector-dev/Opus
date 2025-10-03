use crate::app::{App, Mode, InputMode};
use anyhow::{Result, Context};
use crossterm::event::{KeyCode, KeyEvent};

pub fn key_logic(app: &mut App, key: KeyEvent) -> Result<bool>
{
    match &mut app.input_mode
    {
        InputMode::Normal =>  match app.mode
        {
            Mode::Projects => match key.code
            {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Down => app.next_project(),
                KeyCode::Up => app.previous_project(),
                KeyCode::Enter => app.open_project().context("Failed to Open Project")?,
                KeyCode::Char('a') => app.input_mode = InputMode::EditingProject(String::new()),
                KeyCode::Char('d') => app.delete_project().context("Failed to Delete Project")?,
                _ => {}
            },
            Mode::Tasks => match key.code
            {
                KeyCode::Esc => app.close_project(),
                KeyCode::Down => app.next_task(),
                KeyCode::Up => app.previous_task(),
                KeyCode::Char('t') => app.toggle_task(),
                KeyCode::Char('a') => app.input_mode = InputMode::EditingTask(String::new()),
                KeyCode::Char('d') => app.delete_task().context("Failed to Delete Task")?,
                _ => {}
            },
        },

        InputMode::EditingProject(buffer) => match key.code
        {
            KeyCode::Enter => {let input = buffer.clone(); if !input.is_empty(){app.add_project(&input).context("Failed to read buffer")?;}; app.input_mode = InputMode::Normal;}
            KeyCode::Esc => {app.input_mode = InputMode::Normal;}
            KeyCode::Char(c) => buffer.push(c),
            KeyCode::Backspace => {buffer.pop();}
            _ => {}
        },
        InputMode::EditingTask(buffer) => match key.code
        {
            KeyCode::Enter => {let input = buffer.clone(); if !input.is_empty(){app.add_task(&input).context("Failed to read buffer")?;}; app.input_mode = InputMode::Normal;}
            KeyCode::Esc => {app.input_mode = InputMode::Normal;}
            KeyCode::Char(c) => buffer.push(c),
            KeyCode::Backspace => {buffer.pop();}
            _ => {}
        },

    }
    Ok(false)
}
