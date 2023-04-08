use std::io::{Write, Read};

pub fn edit_note() {
    println!("What Section is Your Note Located?");
    let mut section = String::new();
    std::io::stdin()
        .read_line(&mut section)
        .expect("Failed to read line");

    let section = section.trim();

    println!("What is the name of your note?");

    let mut note_name = String::new();

    std::io::stdin()
        .read_line(&mut note_name)
        .expect("Failed to read line");

    let note_name = note_name.trim();

    let home_dir = dirs::home_dir().unwrap();
    let config_dir = home_dir.join(".config");
    let notes_dir = config_dir.join("notes");
    let tag_dir = notes_dir.join(section);

    let note_file = tag_dir.join(note_name);

    let mut file = std::fs::File::open(note_file.clone()).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    println!("What would you like to change?");
    
    let mut change = String::new();

    std::io::stdin()
        .read_line(&mut change)
        .expect("Failed to read line");

    let change = change.trim();

    match change {
        "n" => {
            println!("What would you like to change the name to?");
            let mut new_name = String::new();
            std::io::stdin()
                .read_line(&mut new_name)
                .expect("Failed to read line");
            let new_name = new_name.trim();
            let new_note_file = tag_dir.join(new_name);
            std::fs::rename(note_file, new_note_file).unwrap();
        }
        "c" => {
            println!("What would you like to change the content to?");
            let mut new_content = String::new();
            std::io::stdin()
                .read_line(&mut new_content)
                .expect("Failed to read line");
            let new_content = new_content.trim();
            let mut file = std::fs::File::create(note_file).unwrap();
            file.write_all(new_content.as_bytes()).unwrap();
        }
        "t" => {
            println!("What would you like to change the tag to?");
            let mut new_tag = String::new();
            std::io::stdin()
                .read_line(&mut new_tag)
                .expect("Failed to read line");
            let new_tag = new_tag.trim();
            let new_tag_dir = notes_dir.join(new_tag);
            if !new_tag_dir.exists() {
                std::fs::create_dir(new_tag_dir.clone()).unwrap();
            }
            let new_note_file = new_tag_dir.join(note_name);
            std::fs::rename(note_file, new_note_file).unwrap();
        }
        _ => {
            println!("[ERROR] Invalid input - use [n]ame, [c]ontent or [t]ag");
        }
    }
    
}