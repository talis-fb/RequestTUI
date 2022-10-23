use std::process::Command;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Tabs},
    Frame, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // std::process::Command::new("/usr/bin/sh")
    //     .arg("-c")
    //     .arg("vim")
    //     .arg("file")
    //     .spawn()
    //     .expect("Error: Failed to run editor")
    //     .wait()
    //     .expect("Error: Editor returned a non-zero status");

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                // Request List Tab
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Tablist
    let tabs_spans = vec![
        Spans::from(vec![Span::from("1 Tab Meu bom")]),
        Spans::from(vec![Span::from("2 Tabs")]),
        Spans::from(vec![Span::from("3 Tab")]),
        Spans::from(vec![Span::from("4 Tab")]),
    ];
    let tabs = Tabs::new(tabs_spans)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Tabs"),
        )
        .select(0)
        // .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);

    // Layout geral
    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // REQUEST
    let request_block = Block::default()
        .borders(Borders::ALL)
        .title("Request")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(request_block, content_layout[0]);

    let request_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(content_layout[0]);

    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Length(7), Constraint::Min(1)].as_ref())
        .split(request_layout[0]);

    let method = Paragraph::new("GET")
        .style(Style::default().bg(Color::Blue).fg(Color::Black))
        .alignment(Alignment::Center);
    f.render_widget(method, header_layout[0]);

    let url = Block::default()
        .borders(Borders::ALL)
        .title("URL")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    f.render_widget(url, header_layout[1]);

    let body = Block::default()
        .borders(Borders::ALL)
        .title("BODY / Headers / Options")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    f.render_widget(body, request_layout[1]);

    // RESPONSE SECTION
    let response_block = Block::default()
        .borders(Borders::ALL)
        .title("Response")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(response_block, content_layout[1]);

    let response_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
        .split(content_layout[1]);

    let status_code = Paragraph::new(" 200 ")
        .style(Style::default().bg(Color::Green).fg(Color::Black))
        .alignment(Alignment::Center);
    f.render_widget(status_code, response_layout[0]);

    let body_response = Block::default()
        .borders(Borders::ALL)
        .title("BODY / Headers / Options")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    f.render_widget(body_response, response_layout[1]);

    // LOG SECTION
    let log_block = Block::default().borders(Borders::TOP).title("Logs");
    f.render_widget(log_block, chunks[2]);
}