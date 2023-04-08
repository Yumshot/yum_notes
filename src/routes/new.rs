use std::{io, fs::File};

use markdown_gen::markdown::{AsMarkdown, Markdown};

pub fn create_new_note() {
    let mut note_name = String::new();
    let mut note_content = String::new();
    let mut tag_name = String::new();

    println!("Enter the name of your note");
    io::stdin()
        .read_line(&mut note_name)
        .expect("Failed to read line");
    println!("Enter the content of your note");
    io::stdin()
        .read_line(&mut note_content)
        .expect("Failed to read line");

    let tags_available = vec!["todo", "note", "reminder"];

    println!("Enter the tag of your note {:?}", tags_available);

    io::stdin()
        .read_line(&mut tag_name)
        .expect("Failed to read line");

    let tag_name = tag_name.trim();

    let home_dir = dirs::home_dir().unwrap();
    let config_dir = home_dir.join(".config");
    let notes_dir = config_dir.join("notes");
    let tag_dir = notes_dir.join(tag_name);

    if !config_dir.exists() {
        std::fs::create_dir(config_dir.clone()).unwrap();
   }

    if !notes_dir.exists() {
        std::fs::create_dir(notes_dir.clone()).unwrap();
    }

    if !tag_dir.exists() {
        std::fs::create_dir(tag_dir.clone()).unwrap();
    }

    let note_name = note_name.trim();
    let note_content = note_content.trim();
    let note_file_path = tag_dir.join(note_name.to_owned() + ".md");
    let file = File::create(note_file_path).unwrap();
    let mut md = Markdown::new(file);

    md.write(note_name.heading(1)).unwrap();
    md.write(tag_name.italic().heading(2)).unwrap();
    md.write(note_content.quote()).unwrap(); 
    }

