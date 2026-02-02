use std::io;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};

#[derive(PartialEq)]
enum InputMode {
    Normal, // no input
    AddingUser,
    AddingTransactionAmount,
    AddingTransactionPayer,
    AddingTransactionEquality,
    RemovingUser,
    RemovingTransaction,
}

pub struct App {
    exit: bool,
    input_mode: InputMode,
    user_input: String,
    transaction_amount_input: String,
    users: crate::Users,
    selected_user_idx: usize, // For selecting user in AddingTransactionFrom
    selected_transaction_idx: usize, // For selecting transaction in RemovingTransaction
    equal_split_selected: bool,
    transaction_history: Vec<String>,
    dong: Vec<String>,
}

pub fn start_tui() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        input_mode: InputMode::Normal,
        user_input: String::new(),
        transaction_amount_input: String::new(),
        users: crate::Users::new(),
        selected_user_idx: 0,
        selected_transaction_idx: 0,
        equal_split_selected: true,
        transaction_history: Vec::new(),
        dong: Vec::new(),
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
                    if self.input_mode != InputMode::AddingUser {
                        self.exit = true;
                    } else {
                        self.user_input.push('q');
                    }
                }
                KeyCode::Char('u') => {
                    if self.input_mode == InputMode::Normal && self.transaction_history.is_empty() {
                        // only allow adding users if no transactions has been recorded
                        self.input_mode = InputMode::AddingUser;
                        self.user_input.clear();
                    }
                }
                KeyCode::Char('r') => {
                    if self.input_mode == InputMode::Normal && !self.users.list_users().is_empty() {
                        // Only allow removing users if no transactions has been recorded
                        // Otherwise, allow removing transactions
                        if self.transaction_history.is_empty() {
                            self.input_mode = InputMode::RemovingUser;
                            self.selected_user_idx = 0;
                        } else {
                            self.input_mode = InputMode::RemovingTransaction;
                            self.selected_transaction_idx = 0;
                        }
                    }
                }
                KeyCode::Char('t') => {
                    if self.input_mode == InputMode::Normal && self.users.list_users().len() > 1 {
                        self.input_mode = InputMode::AddingTransactionAmount;
                        self.transaction_amount_input.clear();
                        self.selected_user_idx = 0;
                    }
                }
                KeyCode::Char('s') => {
                    if self.input_mode == InputMode::Normal && !self.dong.is_empty() {
                        self.users.settle_up();
                        self.transaction_history.clear();
                        self.dong.clear();
                    }
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.selected_user_idx = 0;
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
                                self.input_mode = InputMode::AddingTransactionPayer;
                            }
                        }
                        InputMode::AddingTransactionPayer => {
                            let user_list = self.users.list_users();
                            if !user_list.is_empty() && self.selected_user_idx < user_list.len() {
                                // TODO: Add logic for saving the transaction payer
                            }
                            self.input_mode = InputMode::AddingTransactionEquality;
                        }
                        InputMode::RemovingUser => {
                            let user_list = self.users.list_users();
                            if self.selected_user_idx < user_list.len() {
                                self.users
                                    .remove_user(user_list[self.selected_user_idx].clone());
                            }
                            self.input_mode = InputMode::Normal;
                        }
                        InputMode::AddingTransactionEquality => {
                            let user_list = self.users.list_users();
                            self.input_mode = InputMode::Normal;
                            if self.equal_split_selected {
                                let amount =
                                    self.transaction_amount_input.trim().parse::<f64>().unwrap();
                                self.users.record_payment(
                                    user_list[self.selected_user_idx].as_str(),
                                    amount,
                                );
                                self.transaction_history.push(format!(
                                    "{} paid {} (equally split)",
                                    user_list[self.selected_user_idx], amount
                                ));
                                self.dong = self.users.calculate_total_payments().unwrap();
                            } else {
                                // todo: handle unequal split
                            }
                        }
                        InputMode::RemovingTransaction => {
                            self.users
                                .remove_payment_by_index(self.selected_transaction_idx);
                            self.transaction_history
                                .remove(self.selected_transaction_idx);
                            self.selected_transaction_idx = 0;
                            self.dong = self.users.calculate_total_payments().unwrap();
                            self.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    }
                }
                KeyCode::Up => {
                    if self.input_mode == InputMode::AddingTransactionPayer
                        || self.input_mode == InputMode::RemovingUser
                    {
                        let user_count = self.users.list_users().len();
                        if user_count > 0 {
                            if self.selected_user_idx == 0 {
                                self.selected_user_idx = user_count - 1;
                            } else {
                                self.selected_user_idx -= 1;
                            }
                        }
                    } else if self.input_mode == InputMode::RemovingTransaction {
                        let transaction_count = self.transaction_history.len();
                        if transaction_count > 0 {
                            if self.selected_transaction_idx == 0 {
                                self.selected_transaction_idx = transaction_count - 1;
                            } else {
                                self.selected_transaction_idx -= 1;
                            }
                        }
                    }
                }
                KeyCode::Down => {
                    if self.input_mode == InputMode::AddingTransactionPayer
                        || self.input_mode == InputMode::RemovingUser
                    {
                        let user_count = self.users.list_users().len();
                        if user_count > 0 {
                            self.selected_user_idx = (self.selected_user_idx + 1) % user_count;
                        }
                    } else if self.input_mode == InputMode::RemovingTransaction {
                        let transaction_count = self.transaction_history.len();
                        if transaction_count > 0 {
                            self.selected_transaction_idx =
                                (self.selected_transaction_idx + 1) % transaction_count;
                        }
                    }
                }
                KeyCode::Right | KeyCode::Left => {
                    if self.input_mode == InputMode::AddingTransactionEquality {
                        self.equal_split_selected = !self.equal_split_selected;
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
            .constraints(
                [
                    Constraint::Percentage(35),
                    Constraint::Percentage(35),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(vertical_chunks[1]);

        // For users
        let users_block = {
            let mut block = Block::default()
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
            if matches!(
                self.input_mode,
                InputMode::AddingUser | InputMode::AddingTransactionPayer
            ) {
                block = block.border_style(Style::default().fg(Color::Yellow));
            }
            if matches!(self.input_mode, InputMode::RemovingUser) {
                block = block.border_style(Style::default().fg(Color::Red));
            }
            block
        };
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
            InputMode::RemovingUser => {
                let mut lines: Vec<Line> = Vec::new();
                for (i, u) in user_list.iter().enumerate() {
                    if i == self.selected_user_idx {
                        lines.push(Line::from(Span::styled(
                            format!("> {} <", u),
                            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                        )));
                    } else {
                        lines.push(Line::from(Span::raw(u)));
                    }
                }
                lines.push(Line::from("----------"));
                lines.push(Line::from("< select user to remove >"));
                let text = Text::from(lines);
                Paragraph::new(text)
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
            }
            InputMode::AddingTransactionPayer => {
                // Highlight the selected user
                let mut lines: Vec<Line> = Vec::new();
                for (i, u) in user_list.iter().enumerate() {
                    if i == self.selected_user_idx {
                        lines.push(Line::from(Span::styled(
                            format!("> {} <", u),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )));
                    } else {
                        lines.push(Line::from(Span::raw(u)));
                    }
                }
                lines.push(Line::from("----------"));
                lines.push(Line::from("< select payer of the transaction >"));
                let text = Text::from(lines);
                Paragraph::new(text)
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
            }
            InputMode::AddingTransactionEquality => {
                let mut lines: Vec<Line> = Vec::new();
                for (i, u) in user_list.iter().enumerate() {
                    if i == self.selected_user_idx {
                        lines.push(Line::from(Span::styled(
                            format!("{}", u),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )));
                    } else {
                        lines.push(Line::from(Span::raw(u)));
                    }
                }
                let text = Text::from(lines);
                Paragraph::new(text)
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
            }
            _ => {
                if self.transaction_history.is_empty() {
                    if !lines.is_empty() {
                        lines.push(Line::from("----------"));
                        lines.push(Line::from("< press 'u' to add user | 'r' to remove user >"));
                    } else {
                        lines.push(Line::from("< press 'u' to add user >"));
                    }
                }
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
        let transactions_block = {
            let mut block = Block::default()
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
            if matches!(
                self.input_mode,
                InputMode::AddingTransactionAmount | InputMode::AddingTransactionEquality
            ) {
                block = block.border_style(Style::default().fg(Color::Yellow));
            }
            if matches!(self.input_mode, InputMode::RemovingTransaction) {
                block = block.border_style(Style::default().fg(Color::Red));
            }
            block
        };

        let (transaction_default_text, italic) =
            if self.users.list_users().len() > 1 && self.transaction_history.is_empty() {
                ("< press 't' to add transaction >", false)
            } else if self.users.list_users().len() > 1 && self.transaction_history.len() > 0 {
                (
                    "< press 't' to add transaction | 'r' to remove transaction >",
                    false,
                )
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
            InputMode::AddingTransactionPayer => {
                Paragraph::new(Line::from("> payer: (select user from 'Users' panel)"))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
            }
            InputMode::AddingTransactionEquality => {
                if self.equal_split_selected {
                    let lines = vec![Line::from(vec![
                        Span::from("> split: "),
                        Span::styled(
                            "> equally <",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::from(" unequally"),
                    ])];
                    Paragraph::new(lines)
                        .alignment(Alignment::Left)
                        .wrap(Wrap { trim: true })
                } else {
                    let lines = vec![Line::from(vec![
                        Span::from("> split: equally "),
                        Span::styled(
                            "> unequally <",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ])];
                    Paragraph::new(lines)
                        .alignment(Alignment::Left)
                        .wrap(Wrap { trim: true })
                }
            }
            InputMode::RemovingTransaction => {
                let mut lines: Vec<Line> = Vec::new();
                for (i, u) in self.transaction_history.iter().enumerate() {
                    if i == self.selected_transaction_idx {
                        lines.push(Line::from(Span::styled(
                            format!("> {} <", u),
                            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                        )));
                    } else {
                        lines.push(Line::from(Span::raw(u)));
                    }
                }
                lines.push(Line::from("----------"));
                lines.push(Line::from("< select transaction to remove >"));
                let text = Text::from(lines);
                Paragraph::new(text)
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
            }
            _ => {
                let mut lines: Vec<Line> = self
                    .transaction_history
                    .iter()
                    .map(|u| Line::from(Span::raw(u)))
                    .collect();
                if !lines.is_empty() {
                    lines.push(Line::from("----------"));
                }
                lines.push(Line::from(transaction_default_text));
                Paragraph::new(lines)
                    .add_modifier(if italic {
                        Modifier::ITALIC
                    } else {
                        Modifier::empty()
                    })
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
            }
        };

        let transaction_area = main_chunks[1];
        let transaction_inner = transactions_block.inner(transaction_area);

        frame.render_widget(transactions_block, transaction_area);
        frame.render_widget(transaction_content, transaction_inner);

        let dong_block = {
            let mut block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " Dong ",
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 1,
                    bottom: 1,
                });
            if !self.dong.is_empty() {
                block = block.border_style(Style::default().fg(Color::Cyan));
            }
            block
        };

        let dong = match self.dong.is_empty() {
            true => Paragraph::new(Line::from("Nothing to see here yet!"))
                .alignment(Alignment::Left)
                .add_modifier(Modifier::ITALIC)
                .wrap(Wrap { trim: true }),
            false => {
                let other = vec![
                    Line::from("----------"),
                    Line::from("< press 's' to settle up payments >"),
                ];
                let lines: Vec<Line> = self
                    .dong
                    .iter()
                    .map(|u| Line::from(Span::raw(u)))
                    .chain(other)
                    .collect();
                Paragraph::new(lines)
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
            }
        };

        let dong_area = main_chunks[2];
        let dong_inner = dong_block.inner(dong_area);
        frame.render_widget(dong_block, dong_area);
        frame.render_widget(dong, dong_inner);
    }
}
