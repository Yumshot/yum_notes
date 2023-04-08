use std::io;

mod routes;

fn main() {
    println!("What would you like to do?");
    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read line");
    let user_choice = user_choice.trim();

        match user_choice {
            "n" => {
                routes::new::create_new_note();
            }
            "e" => {
                routes::edit::edit_note();
            }
            "d" => {
                routes::delete::delete_note();
            }
            "l" => {
                println!("You chose 3");
            }
            "q" => {
                println!("You chose 4");
            }
            _ => {
                println!("[ERROR] Invalid input - use [n]ew, [e]dit, [d]elete, [l]ist or [q]uit");
            }
        }
    
}
