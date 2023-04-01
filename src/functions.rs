use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Note {
    pub name: String,
    pub tag: TagEnum,
    pub content: String,
    pub status: NoteEnum,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TagEnum {
    Feature,
    Bug,
    Enhancement,
    Question,
    Other,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum NoteEnum {
    Todo,
    Working,
    Done,
    Deleted,
}

pub fn log(message: String) {
    println!("{}", message);
}

/// > This function takes a string as an argument and returns a Result type that contains either a
/// string or an error.
///
/// Arguments:
///
/// * `search_path`: This is the environment variable that we're searching for.
///
/// Returns:
///
/// A Result<String, std::io::Error>
fn gather_user_home_directory(search_path: String) -> Result<String, std::io::Error> {
    let home_dir = std::env::var(search_path).unwrap();
    Ok(home_dir)
}

/// This function creates a config directory and a notes directory if they don't exist. It also creates
/// a config file if it doesn't exist.
fn find_config_directory() {
    let mut config_dir = gather_user_home_directory("HOME".to_string()).unwrap();
    let mut note_dir = gather_user_home_directory("HOME".to_string()).unwrap();
    config_dir = format!("{}{}", config_dir, "/.config/yum_notes");
    note_dir = format!("{}{}", note_dir, "/.config/yum_notes/notes");

    //TODO: add error handling for this || split this into its own function
    if !PathBuf::from(&config_dir).exists() {
        std::fs::create_dir_all(&config_dir).unwrap();
        log(format!("Created config directory: {:?}", config_dir));
    }

    //TODO: add error handling for this || split this into its own function
    if PathBuf::from(&config_dir).exists() {
        log(format!("Config directory exists: {:?}", config_dir));
        std::fs::create_dir_all(note_dir.clone()).unwrap();
        log(format!("Created notes directory: {:?}", note_dir));
        let config_file = format!("{}{}", config_dir, "/config.json");

        if !PathBuf::from(&config_file).exists() {
            let mut file = std::fs::File::create(&config_file).unwrap();
            let mut option_map = HashMap::new();
            option_map.insert("color", "system"); //  options are light | dark | system
            option_map.insert("seperator", "---");
            let json = serde_json::to_string_pretty(&option_map).unwrap();
            file.write_all(json.as_bytes()).unwrap();
            log(format!("Created config file: {:?}", config_file));
        } else {
            log(format!("Config file exists: {:?}", config_file));
        }
    }
}

/// It takes a note struct, creates a file name, creates a file path, checks if the file exists, if it
/// doesn't, it creates the file, and writes the note to the file.
///
/// The function is pretty simple, but it's a good example of how to use the `std::fs` module.
///
/// The `std::fs` module is a module that contains functions for interacting with the file system.
///
/// The `std::fs::File` struct is a struct that represents a file.
///
/// The `std::fs::File::create` function is a function that creates a file.
///
/// The `std::fs::File::write_all` function is a function that writes to a file.
///
/// The `std::fs::File::write_all` function takes a byte array as an argument.
///
/// The `serde_json::to_string_pretty`
///
/// Arguments:
///
/// * `note_structured`: Note
pub fn create_note(note_structured: Note) {
    let mut note_dir = gather_user_home_directory("HOME".to_string()).unwrap();
    note_dir = format!("{}{}", note_dir, "/.config/yum_notes/notes/");
    let file_name = format!("{}{}", note_structured.name, ".md");
    let note_file = format!("{}{}", note_dir, file_name);

    if !PathBuf::from(&note_file).exists() {
        let mut file = std::fs::File::create(&note_file).unwrap();
        let mut note_map = HashMap::new();
        note_map.insert("content", note_structured.content);
        match note_structured.tag {
            TagEnum::Feature => {
                note_map.insert("tag", "Feature".to_owned());
            }
            TagEnum::Bug => {
                note_map.insert("tag", "Bug".to_owned());
            }
            TagEnum::Enhancement => {
                note_map.insert("tag", "Enhancement".to_owned());
            }
            TagEnum::Question => {
                note_map.insert("tag", "Question".to_owned());
            }
            TagEnum::Other => {
                note_map.insert("tag", "Other".to_owned());
            }
        }
        match note_structured.status {
            NoteEnum::Todo => {
                note_map.insert("status", "📦".to_owned());
            }
            NoteEnum::Working => {
                note_map.insert("status", "⚙️".to_owned());
            }
            NoteEnum::Done => {
                note_map.insert("status", "✔️".to_owned());
            }
            NoteEnum::Deleted => {
                note_map.insert("status", "🗙".to_owned());
            }
        }

        // TODO: Write a function to convert the note into a markdown format.
        let json = serde_json::to_string_pretty(&note_map).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        log(format!("Created note file: {:?}", note_file));
    } else {
        log(format!("Note file exists: {:?}", note_file));
    }
}

/// This function takes in a note name, note content, and note status and updates the note file with the
/// new content and status.
///
/// Arguments:
///
/// * `note_name`: The name of the note you want to edit
/// * `note_content`: String, note_status: NoteEnum
/// * `note_status`: NoteEnum::Todo,
pub fn edit_note(note_name: String, note_content: String, note_status: NoteEnum) {
    let mut note_dir = gather_user_home_directory("HOME".to_string()).unwrap();
    note_dir = format!("{}{}", note_dir, "/.config/yum_notes/notes/");
    let file_name = format!("{}{}", note_name, ".md");
    let note_file = format!("{}{}", note_dir, file_name);

    if PathBuf::from(note_file.clone()).exists() {
        let mut read_file = std::fs::File::open(&note_file).unwrap();

        let mut note_map = HashMap::new();

        let mut file_contents = String::new();
        read_file.read_to_string(&mut file_contents).unwrap();
        let json: HashMap<String, String> = serde_json::from_str(&file_contents).unwrap();

        if json["content"] != note_content {
            note_map.insert("content", note_content);
        } else {
            note_map.insert("content", json["content"].clone());
        }

        match note_status {
            NoteEnum::Todo => {
                if json["status"] != "📦" {
                    note_map.insert("status", "📦".to_owned());
                } else {
                    note_map.insert("status", json["status"].clone());
                }
            }
            NoteEnum::Working => {
                if json["status"] != "⚙️" {
                    note_map.insert("status", "⚙️".to_owned());
                } else {
                    note_map.insert("status", json["status"].clone());
                }
            }
            NoteEnum::Done => {
                if json["status"] != "✔️" {
                    note_map.insert("status", "✔️".to_owned());
                } else {
                    note_map.insert("status", json["status"].clone());
                }
            }
            NoteEnum::Deleted => {
                if json["status"] != "🗙" {
                    note_map.insert("status", "🗙".to_owned());
                } else {
                    note_map.insert("status", json["status"].clone());
                }
            }
        }

        let json = serde_json::to_string_pretty(&note_map).unwrap();
        let mut file = std::fs::File::create(&note_file).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        log(format!("Edited note file: {:?}", note_file));
    } else {
        log(format!("Note file does not exist: {:?}", note_file));
    }
}

/// It deletes a note file from the notes directory.
///
/// Arguments:
///
/// * `note_name`: The name of the note to delete.
pub fn delete_note(note_name: String) {
    let mut note_dir = gather_user_home_directory("HOME".to_string()).unwrap(); // get to working directory
    note_dir = format!("{}{}", note_dir, "/.config/yum_notes/notes/"); // add folder to string
    let file_name = format!("{}{}", note_name, ".md"); // add file name to path
    let note_file = format!("{}{}", note_dir, file_name); // add file name to path
    if PathBuf::from(note_file.clone()).exists() {
        // delete the file
        log(format!("Deleted note file: {:?}", note_file.clone()));
        std::fs::remove_file(note_file).unwrap();
    } else {
        log(format!("Note file does not exist: {:?}", file_name));
    }
}
