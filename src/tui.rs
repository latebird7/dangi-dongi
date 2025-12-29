use std::io;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};

pub struct App {
    exit: bool,
}

pub fn start_tui() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App { exit: false };

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }

            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let size = frame.area();

        // Draw outer frame
        let block = Block::default().borders(Borders::ALL).title(Span::styled(
            " Dangi-Dongi ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ));
        frame.render_widget(block, size);

        // Layout: vertical split for welcome and main area
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(2)
            .vertical_margin(4)
            .constraints(
                [
                    Constraint::Length(4), // Welcome message height
                    Constraint::Min(0),    // Main area
                ]
                .as_ref(),
            )
            .split(size);

        let welcome = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "Welcome to Dangi-Dongi!",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
        ])
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
        frame.render_widget(welcome, vertical_chunks[0]);

        // Main area: horizontal split for Users and Transactions
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(vertical_chunks[1]);

        // For users
        let users_block = Block::default().borders(Borders::ALL).title(Span::styled(
            " Users ",
            Style::default().add_modifier(Modifier::BOLD),
        ));
        frame.render_widget(users_block, main_chunks[0]);

        // For transactions
        let transactions_block = Block::default().borders(Borders::ALL).title(Span::styled(
            " Transactions ",
            Style::default().add_modifier(Modifier::BOLD),
        ));
        frame.render_widget(transactions_block, main_chunks[1]);
    }
}
