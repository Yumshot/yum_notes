use std::io;

use crossterm::event::{Event, KeyCode, self};
use tui::{backend::Backend, Terminal, Frame, widgets::{Block, Borders, BorderType, ListItem, List}, layout::{Alignment, Direction, Layout, Constraint}};





pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                // KeyCode::Up => todo!(),
                // KeyCode::Down => todo!(),
                // KeyCode::Enter => println!("Selected item: {}", "TODO"),
                _ => {}
        }
    }
}
}

pub fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("🗇 Yum Notes 🗇")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);  

      // Create a list
      let items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ];
    let list = List::new(items);

    // Create a layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    // Render the list
    f.render_widget(list, chunks[0]);
}

