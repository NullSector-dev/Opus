use crate::{app::{App, Mode, InputMode}, banner::OPUS_BANNER};
use ratatui::{ layout::{Constraint, Direction, Layout, Rect}, style::{Style, Modifier}, widgets::{Block, Borders, Paragraph, List, ListItem, Clear}, Frame, text::Span};

pub fn draw(frame: &mut Frame, app: &App)
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints
        (
            [
                Constraint::Length(16),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
        )
        .split(frame.area());
    draw_banner(frame, chunks[0]);

    match app.mode
    {
        Mode::Projects => draw_projects(frame, chunks[1], app),
        Mode::Tasks => draw_tasks(frame, chunks[1], app)
    }

    match &app.input_mode
    {
        InputMode::EditingProject(buf) => draw_popup(frame, "New Project", buf),
        InputMode::EditingTask(buf) => draw_popup(frame, "New Task", buf),
        _ => {}
    }

    draw_help(frame, chunks[2], app);
}

fn draw_banner(frame: &mut Frame, area: ratatui::layout::Rect)
{
    let paragraph = Paragraph::new(OPUS_BANNER)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(paragraph, area);
}

fn draw_help(frame: &mut Frame, area: ratatui::layout::Rect, app: &App)
{
    let help_text = match app.mode
    {
        Mode::Projects => "[  /  ] Move | [ENTER] Open Project | [a] Add Project | [d] Delete Project | [q] Quit",
        Mode::Tasks => "[  /  ] Move | [t] Toggle Task | [a] Add task | [d] Delete Task | [Esc] Back",
    };

    let paragraph = Paragraph::new(Span::from(help_text))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}

fn draw_projects(frame: &mut Frame, area: ratatui::layout::Rect, app: &App)
{
    let items: Vec<ListItem> = app.projects.iter().map(|p| ListItem::new(format!("{}", p.name))).collect();

    let list = List::new(items)
        .block(Block::default().title(" Projects ").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::SLOW_BLINK))
        .highlight_symbol("-> ");

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.selected));
    frame.render_stateful_widget(list, area, &mut state);

}

fn draw_tasks(frame: &mut Frame, area: ratatui::layout::Rect, app: &App)
{
    let items: Vec<ListItem> = app.tasks
                                  .iter()
                                  .map(|t| { let status = if t.done { "[X]" } else { " " }; ListItem::new(format!("{} {}", t.title, status)) })
                                  .collect();
    let title = app.current_project.as_ref().map(|p| format!(" {} - To-DO ", p.name)).unwrap_or_else(|| "Tasks".to_string());

    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::SLOW_BLINK))
        .highlight_symbol("-> ");

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.selected));
    frame.render_stateful_widget(list, area, &mut state);
}

fn  draw_popup(frame: &mut Frame, title: &str, buffer: &str)
{
    let area = centered_popup(30, 3, frame.area());
    let paragraph = Paragraph::new(buffer.to_string()).block(Block::default().title(title).borders(Borders::ALL));
    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

fn centered_popup(per_x: u16, per_y: u16, r: Rect) -> Rect
{
    let popup_width = r.width * per_x / 100;
    let popup_height = r.width * per_y / 100;

    let popup_x = r.width.saturating_sub(popup_width) / 2;
    let popup_y = r.height.saturating_sub(popup_height) / 2;

    Rect
    {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    }
}
