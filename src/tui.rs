use std::io;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};

pub struct App {
    exit: bool,
    input_mode: bool,
    user_input: String,
}

pub fn start_tui() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        input_mode: false,
        user_input: String::new(),
    };

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
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('q') => {
                    if !self.input_mode {
                        self.exit = true;
                    } else {
                        self.user_input.push('q');
                    }
                }
                KeyCode::Char('u') => {
                    self.input_mode = true;
                    self.user_input.clear();
                }
                KeyCode::Esc => {
                    self.input_mode = false;
                }
                KeyCode::Enter => {
                    if self.input_mode {
                        self.input_mode = false;
                    }
                }
                KeyCode::Backspace => {
                    if self.input_mode {
                        self.user_input.pop();
                    }
                }
                KeyCode::Char(c) => {
                    if self.input_mode {
                        self.user_input.push(c);
                    }
                }
                _ => {}
            }
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
            .vertical_margin(2)
            .constraints(
                [
                    Constraint::Length(2), // Welcome message height
                    Constraint::Min(0),    // Main area
                ]
                .as_ref(),
            )
            .split(size);

        let welcome = Paragraph::new(vec![
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
        let users_block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                " Users ",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .padding(Padding {
                left: 1,
                right: 1,
                top: 1,
                bottom: 1,
            });
        let users_content = if self.input_mode {
            Paragraph::new(vec![Line::from(format!("> {}", self.user_input.as_str()))])
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
        } else {
            Paragraph::new(vec![Line::from("< press 'u' to add user >")])
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
        };

        let users_area = main_chunks[0];
        let users_inner = users_block.inner(users_area);
        frame.render_widget(users_block, users_area);

        frame.render_widget(users_content, users_inner);

        // For transactions
        let transactions_block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                " Transactions ",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .padding(Padding {
                left: 1,
                right: 1,
                top: 1,
                bottom: 1,
            });

        let transaction_content = Paragraph::new(vec![Line::from(
            "Please add at least two users to start recording transactions.",
        )])
        .add_modifier(Modifier::ITALIC)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

        let transaction_area = main_chunks[1];
        let transaction_inner = transactions_block.inner(transaction_area);

        frame.render_widget(transactions_block, transaction_area);
        frame.render_widget(transaction_content, transaction_inner);
    }
}
