use std::env;

use functions::TagEnum;

mod functions;
// todo!()
fn main() {
    let args: Vec<String> = env::args().collect();
    // NOTE: has to be called as -- -n in order to work at the moment.
    let possibles = vec!["-n", "-e", "-d", "-l", "-s"];
    if args.len() < 1 {
        println!("Please provide a command");
        std::process::exit(1);
    } 
    
    if possibles.contains(&args[1].as_str()) {
        let args_tag = check_tag(args[3].clone());
       

        match args[1].as_str() {
            "-n" => {
                let note_structured = functions::Note {
                    name: args[2].clone().to_string(),
                    tag: args_tag,
                    content: args[4].clone().to_string(),
                    status: functions::NoteEnum::Todo,
                };
                functions::create_note(note_structured);
            },
            "-e" => {
                functions::edit_note(
                    args[2].clone(),
                    args[4].clone(),
                    functions::NoteEnum::Todo,
                );
            }
            "-d" => {
                functions::delete_note(args[1].clone());
            },

            //TODO: Implement the rest of the functions.
            // "-l" => functions::list_notes(),
            // "-s" => functions::search_notes(),

            _ => {
                println!("Please provide a valid command");
                std::process::exit(1);
            },
        }
    } else {
        println!("Please provide a valid command");
    }
    
}

fn check_tag(args: String) -> TagEnum {
    match args.as_str() {
        "-t" => functions::TagEnum::Feature,
        "-b" => functions::TagEnum::Bug,
        "-e" => functions::TagEnum::Enhancement,
        "-q" => functions::TagEnum::Question,
        _ => functions::TagEnum::Other,
    }
}