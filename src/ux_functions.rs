use std::io;

use crossterm::event::{Event, KeyCode, self};
use tui::{backend::Backend, Terminal, Frame, widgets::{Block, Borders, BorderType, ListItem, List, Tabs, Wrap, Paragraph}, layout::{Alignment, Direction, Layout, Constraint}, style::{Style, Color, Modifier}, text::{Span, Spans}};

pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> App<'a> {
   

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub fn create_new_app() -> App<'static> {
    App {
        titles: vec!["Notes", "Add", "Search", "Delete", "Edit"],
        index: 1,
    }
}


pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                _ => {}
            }
        }
    }
}


fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);
    let main_block = Block::default()
    .borders(Borders::ALL)
    .title("🗇 Yum Notes 🗇")
    .title_alignment(Alignment::Center)
    .border_type(BorderType::Rounded);
    f.render_widget(main_block, size);

    
    let block = Block::default().style(Style::default().bg(Color::DarkGray).fg(Color::White));
    f.render_widget(block, size);
    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Cyan)),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

     let inner = match app.index {
        0 => {
            let item_path = "/home/yumshot/.config/yum_notes/notes";
            let items = std::fs::read_dir(item_path)
                .unwrap()
                .map(|res| res.map(|e| ListItem::new(e.file_name().into_string().unwrap())))
                .collect::<Result<Vec<ListItem>, io::Error>>()
                .unwrap(); 
            let list = List::new(items);
            f.render_widget(list, inner_chunks[0]);
            Block::default().title("Overview").borders(Borders::ALL)
        },
        1 => {
            
            let input = Paragraph::new("Add a note here")
                .block(Block::default().borders(Borders::ALL).title("Create a New Note").title_alignment(Alignment::Center).border_type(BorderType::Rounded))
                .style(Style::default().fg(Color::White).bg(Color::DarkGray))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(input, inner_chunks[0]);
            Block::default()
        },
        2 => Block::default().title("Search Note **").borders(Borders::ALL),
        3 => Block::default().title("Delete Note").borders(Borders::ALL),
        4 => Block::default().title("Edit Note").borders(Borders::ALL),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);
}
