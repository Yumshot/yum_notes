use std::io;

use crossterm::event::{Event, KeyCode, self};
use tui::{backend::Backend, Terminal, Frame, widgets::{Block, Borders, BorderType, ListItem, List, Tabs, Row, Paragraph, Wrap}, layout::{Alignment, Direction, Layout, Constraint}, style::{Style, Color, Modifier}, text::{Span, Spans, Text}};

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
                KeyCode::Enter => {
                    match app.index {
                        0 => {
                                print!("Targetting Notes");
                        },
                        1 => {
                                print!("Targetting Add");
                        },
                        2 => {
                                print!("Targetting Search");
                        },
                        3 => {
                                print!("Targetting Delete");
                        },
                        4 => {
                                print!("Targetting Edit");
                        },
                        _ => {
                                print!("Targetting Default");
                        }
                }
                },
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
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Tabs"))
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
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

     let inner = match app.index {
        0 => {
        // Bottom two inner blocks
        let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Bottom left block with all default borders
    // let block = Block::default();
    // f.render_widget(block, bottom_chunks[0]);
            let item_path = "/home/yumshot/.config/yum_notes/notes";
            let items = std::fs::read_dir(item_path)
                .unwrap()
                .map(|res| res.map(|e| ListItem::new(e.file_name().into_string().unwrap())))
                .collect::<Result<Vec<ListItem>, io::Error>>()
                .unwrap(); 
            let list = List::new(items).block(Block::default().borders(Borders::NONE).border_type(BorderType::Rounded));
            
            f.render_widget(list, inner_chunks[0]);


    // Bottom right block with styled left and right border
    let block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT).inner(f.size());
    // f.render_widget(block, bottom_chunks[1]);
            let item_path = "/home/yumshot/.config/yum_notes/notes";
            // loop through all the files in the item_path

            for entry in std::fs::read_dir(item_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                
                //read the contents of entrys that arent directories
                if path.is_file() {
                     let contents = std::fs::read_to_string(path);

                    for line in contents.unwrap().lines() {
                        let text = Text::from(line);
                        let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::NONE).border_type(BorderType::Rounded)).wrap(Wrap { trim: false });
                        f.render_widget(paragraph, inner_chunks[1]);
                    }

                    // let contents = std::fs::read_to_string(path)
                    //     .expect("Something went wrong reading the file");
                    // let text = Text::from(contents);
                    // let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::NONE).border_type(BorderType::Rounded)).wrap(Wrap { trim: true });
                    // f.render_widget(paragraph, inner_chunks[1]);
                }

            }


            Block::default().title("⭐ Overview ⭐").borders(Borders::ALL).border_type(BorderType::Rounded)
        },
        1 => {
            // create a user input box, 2 drop down menus, and a submit button
            let input = Paragraph::new("Name of Note").block(Block::default().borders(Borders::empty())).alignment(Alignment::Center);
            f.render_widget(input, inner_chunks[0]);

            Block::default().title("⭐ Add Note ⭐").borders(Borders::ALL).border_type(BorderType::Rounded)

        },
        2 => Block::default().title("⭐ Search Note ⭐").borders(Borders::ALL).border_type(BorderType::Rounded),
        3 => Block::default().title("⭐ Delete Note ⭐").borders(Borders::ALL).border_type(BorderType::Rounded),
        4 => Block::default().title("⭐ Edit Note ⭐").borders(Borders::ALL).border_type(BorderType::Rounded),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);
}
