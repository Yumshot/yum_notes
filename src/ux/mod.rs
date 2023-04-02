use std::io;

use crossterm::event::{Event, KeyCode, self};
use tui::{backend::Backend, Terminal, Frame, widgets::{Block, Borders, BorderType, ListItem, List, Tabs}, layout::{Alignment, Direction, Layout, Constraint, Rect}, style::{Style, Color, Modifier}, text::{Span, Spans}};

pub(crate) mod structs;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: structs::App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                KeyCode::Enter => {
                    match app.index {
                        0 => {
                            println!("Notes");
                        }
                        1 => {
                            println!("Add");
                        }
                        2 => {
                            println!("Search");
                        }
                        3 => {
                            println!("Delete");
                        }
                        4 => {
                            println!("Edit");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &structs::App) {
    create_main_window(f, app);
}




fn create_main_window(f: &mut Frame<impl Backend>, app: &structs::App) {
    let size = f.size();
    let mod_height = size.height - 40;
    let mod_width = size.width;
    let size = Rect::new(0, 0, mod_width, mod_height);


    // NOTE: MAIN LAYOUT 
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    // NOTE: OUTER BORDER
    let main_block = Block::default()
    .borders(Borders::ALL)
    .title("🗇 Yum Notes 🗇")
    .title_alignment(Alignment::Center)
    .border_type(BorderType::Rounded);
    f.render_widget(main_block, size);
    let block = Block::default().style(Style::default().bg(Color::DarkGray).fg(Color::White));
    f.render_widget(block, size);

    // NOTE:  TITILES WITHIN TABS
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

    // NOTE: TABS
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs").border_type(BorderType::Rounded))
        .select(app.index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);

    // NOTE: INNER LAYOUT - CHUNKS INSIDE THE PANEL
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

// NOTE: INNER BLOCKS - INFORMATION INSIDE THE PANEL
     let inner = match app.index {
        0 => {
            let item_path = "/home/yumshot/.config/yum_notes/notes";

            let items = std::fs::read_dir(item_path)
                .unwrap()
                .map(|res| res.map(|e| ListItem::new(e.file_name().into_string().unwrap())))
                .collect::<Result<Vec<ListItem>, io::Error>>()
                .unwrap();
            let list = List::new(items)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");
            f.render_widget(list, inner_chunks[0]);

            Block::default().title("Overview").borders(Borders::ALL).border_type(BorderType::Rounded)
        },
        1 => {
            
         
            Block::default()
        },
        2 => Block::default().title("Search Note **").borders(Borders::ALL).border_type(BorderType::Rounded),
        3 => Block::default().title("Delete Note").borders(Borders::ALL).border_type(BorderType::Rounded),
        4 => Block::default().title("Edit Note").borders(Borders::ALL).border_type(BorderType::Rounded),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);
}