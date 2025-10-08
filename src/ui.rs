use crate::{app::{App, Mode, InputMode}, banner::OPUS_BANNER};
use ratatui::{ layout::{Constraint, Direction, Layout, Rect}, style::{Style, Modifier, Color}, widgets::{Block, Borders, Paragraph, List, ListItem, Clear}, Frame, text::{Line, Span, Text}};

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
        InputMode::EditingProject(buf) => draw_popup(frame, " Add New Project ", buf, "Add Project"),
        InputMode::EditingTask(buf) => draw_popup(frame, " Add New Task ", buf, "Add Task"),
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
        Mode::Tasks => "[  /  ] Move | [ENTER] Toggle Task | [a] Add task | [d] Delete Task | [Esc] Back",
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
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::White))
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
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::White))
        .highlight_symbol("-> ");

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.selected));
    frame.render_stateful_widget(list, area, &mut state);
}

fn  draw_popup(frame: &mut Frame, title: &str, buffer: &str, help_add: &str)
{
    let area = centered_popup(30, 7, frame.area());
    let chunks = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(5), Constraint::Length(3)].as_ref()).split(area);
    let wrap = wrap_text(buffer, (area.width - 2) as usize);
    let input = Paragraph::new(wrap).block(Block::default().title(title).borders(Borders::ALL).style(Style::default().bg(Color::Black).fg(Color::White)));
    let help_text = Line::from(vec![
        Span::styled("[Enter] ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(help_add),
        Span::styled(" | [Esc]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Quit"),
    ]);
    let help = Paragraph::new(Text::from(help_text)).block(Block::default().borders(Borders::ALL).style(Style::default().bg(Color::Black).fg(Color::White)));
    frame.render_widget(Clear, area);
    frame.render_widget(input, chunks[0]);
    frame.render_widget(help, chunks[1]);
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
fn wrap_text(content: &str, max_width: usize) -> Text<'_>
{
    let mut lines = Vec::new();
    let mut currentln = String::new();

    for word in content.split_whitespace()
    {
        if currentln.len() + word.len() + 1 > max_width{lines.push(Line::from(currentln.trim_end().to_string())); currentln.clear();}
        currentln.push_str(word);
        currentln.push(' ');
    }

    if !currentln.is_empty(){lines.push(Line::from(currentln.trim_end().to_string()));}
    ratatui::text::Text::from(lines)
}
