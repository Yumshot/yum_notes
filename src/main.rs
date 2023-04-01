use std::env;

mod functions;

fn main() {
    let args: Vec<String> = env::args().collect();
    // NOTE: has to be called as -- -n in order to work at the moment.
    let possibles = vec!["-n", "-e", "-d", "-l", "-s"];
    if args.len() > 1 {
        for arg in args.iter() {
            if possibles.contains(&arg.as_str()) {
                match arg.as_str() {
                    "-n" => {
                        let note_structured = functions::Note {
                            name: args[2].clone(),
                            content: args[3].clone(),
                            status: functions::NoteEnum::Todo,
                        };
                        functions::create_note(note_structured);
                    }
                    "-e" => {
                        functions::edit_note(
                            args[2].clone(),
                            args[3].clone(),
                            functions::NoteEnum::Todo,
                        );
                    }
                    "-d" => {
                        functions::delete_note(args[1].clone());
                    }
                    // "list" => {
                    //     list_notes();
                    // }
                    // "search" => {
                    //     search_notes();
                    // }
                    _ => {
                        panic!("Command not found in yum_note.");
                    }
                }
            } else {
                panic!("Not a possible argument for note command: {}", arg);
            }
        }
    } else {
        panic!("No argument provided for note command.")
    }
}
