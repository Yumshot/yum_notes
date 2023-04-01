use std::io;

use crossterm::event::{Event, KeyCode, self};
use tui::{backend::Backend, Terminal, Frame, widgets::{Block, Borders, BorderType, ListItem, List, Tabs}, layout::{Alignment, Direction, Layout, Constraint}, style::{Style, Color, Modifier}, text::{Span, Spans}};

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
        titles: vec!["Notes", "Productivity", "Github", "Documentation"],
        index: 0,
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

    
    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);
    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);
    let inner = match app.index {
        0 => Block::default().title("Inner 0").borders(Borders::ALL),
        1 => Block::default().title("Inner 1").borders(Borders::ALL),
        2 => Block::default().title("Inner 2").borders(Borders::ALL),
        3 => Block::default().title("Inner 3").borders(Borders::ALL),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);
}



// pub fn ui<B: Backend>(f: &mut Frame<B>) {
//     // Wrapping block for a group
//     // Just draw the block and the group on the same area and build the group
//     // with at least a margin of 1
//     let size = f.size();

//     // Surrounding block
  

//       // Create a list
//       let items = vec![
//         ListItem::new("Item 1"),
//         ListItem::new("Item 2"),
//         ListItem::new("Item 3"),
//     ];
//     let list = List::new(items);

//     // Create a layout
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(3)
//         .constraints([Constraint::Percentage(100)].as_ref())
//         .split(size);

//     // Render the list
//     f.render_widget(list, chunks[0]);
// }

