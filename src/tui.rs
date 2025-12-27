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
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q'){
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

        // Center area for welcome message
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);

        let welcome = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "Welcome to Dangi-Dongi!",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from("Press 'q' or Esc to quit."),
        ])
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

        frame.render_widget(welcome, chunks[0]);
    }
}
