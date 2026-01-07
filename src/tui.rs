use std::io;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};

#[derive(PartialEq)]
enum InputMode {
    Normal,  // no input
    AddingUser,
    AddingTransactionAmount,
}

pub struct App {
    exit: bool,
    input_mode: InputMode,
    user_input: String,
    transaction_amount_input: String,
    users: crate::Users,
}

pub fn start_tui() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        input_mode: InputMode::Normal,
        user_input: String::new(),
        transaction_amount_input: String::new(),
        users: crate::Users::new(),
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
                    if self.input_mode == InputMode::Normal {
                        self.exit = true;
                    } else {
                        self.user_input.push('q');
                    }
                }
                KeyCode::Char('u') => {
                    self.input_mode = InputMode::AddingUser;
                    self.user_input.clear();
                }
                KeyCode::Char('t') => {
                    // TODO: uncomment when transaction logic is completed
                    // if self.users.list_users().len() > 1 {
                    self.input_mode = InputMode::AddingTransactionAmount;
                    self.transaction_amount_input.clear();
                    // }
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Enter => {
                    match self.input_mode {
                        InputMode::AddingUser => {
                            let name = self.user_input.trim();
                            if !name.is_empty() {
                                self.users.add_user(name.to_string());
                            }
                            self.input_mode = InputMode::Normal;
                            self.user_input.clear();
                        }
                        InputMode::AddingTransactionAmount => {
                            let transaction = self.transaction_amount_input.trim();
                            if !transaction.is_empty() {
                                // Placeholder for adding transaction logic
                            }
                            self.input_mode = InputMode::Normal;
                            self.transaction_amount_input.clear();
                        }
                        _ => {}
                    }
                }
                KeyCode::Backspace => match self.input_mode {
                    InputMode::AddingUser => {
                        self.user_input.pop();
                    }
                    InputMode::AddingTransactionAmount => {
                        self.transaction_amount_input.pop();
                    }
                    _ => {}
                },
                KeyCode::Char(c) => {
                    if self.input_mode == InputMode::AddingUser {
                        self.user_input.push(c);
                    } else if self.input_mode == InputMode::AddingTransactionAmount {
                        // Allow only digits and one decimal point
                        if c.is_ascii_digit() {
                            self.transaction_amount_input.push(c);
                        } else if c == '.' && !self.transaction_amount_input.contains('.') {
                            self.transaction_amount_input.push(c);
                        }
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

        let welcome = Paragraph::new(vec![Line::from(vec![Span::styled(
            "Welcome to Dangi-Dongi!",
            Style::default().add_modifier(Modifier::BOLD),
        )])])
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
        let user_list = self.users.list_users();
        let mut lines: Vec<Line> = user_list.iter().map(|u| Line::from(Span::raw(u))).collect();

        let users_content = match self.input_mode {
            InputMode::AddingUser => {
                lines.push(Line::from(format!("> {}", self.user_input.as_str())));
                let text = Text::from(lines);
                Paragraph::new(text)
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
            }
            _ => {
                lines.push(Line::from("< press 'u' to add user >"));
                let text = Text::from(lines);
                Paragraph::new(text)
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
            }
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

        let (transaction_default_text, italic) = if self.users.list_users().len() > 1 {
            ("< press 't' to add transaction >", false)
        } else {
            (
                "Please add at least two users to start recording transactions.",
                true,
            )
        };
        let transaction_content = match self.input_mode {
            InputMode::AddingTransactionAmount => Paragraph::new(Line::from(format!(
                "> amount: {}",
                self.transaction_amount_input.as_str()
            )))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true }),
            _ => Paragraph::new(vec![Line::from(transaction_default_text)])
                .add_modifier(if italic {
                    Modifier::ITALIC
                } else {
                    Modifier::empty()
                })
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true }),
        };

        let transaction_area = main_chunks[1];
        let transaction_inner = transactions_block.inner(transaction_area);

        frame.render_widget(transactions_block, transaction_area);
        frame.render_widget(transaction_content, transaction_inner);
    }
}
