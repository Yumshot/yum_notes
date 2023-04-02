use gtk::{prelude::*, gdk, CssProvider};
use gtk::{Application, ApplicationWindow, Button};
use gtk4 as gtk;




fn main() -> glib::ExitCode {
    let buttons = ["Notes", "Tasks", "Github", "Settings", "Quit"];
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    );

    
    
    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        
        
        
        //create a css provider to add a background color to the window
        let provider = CssProvider::new();
        

        //add the css provider to the window
        gtk::StyleContext::add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        //add the css to the provider
        provider.load_from_path("style.scss");

        // add a class to the window
        window.set_css_classes(&["body"]);
        window.set_title(Some("YUMSHOT"));
        window.set_default_size(350, 70);


        //create a horizontal box to hold our buttons and add it to the window
        let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 15);
        // center the button box
        button_box.set_halign(gtk::Align::Center);
        button_box.set_valign(gtk::Align::Center);
        // add the buttons to the button box
        for button in buttons.iter() {
            let button = Button::with_label(button);
            button_box.append(&button);

            //add a class to the button
            button.set_css_classes(&["custom-btn"]);

            // clone the buttons so we can connect functions to them
            let button_clone = button.clone();
            let window_clone = window.clone();

            //connect the buttons to functions
            button.connect_clicked(move |_| {
                // println!("{} button clicked", button_clone.label().unwrap());
                match button_clone.label().unwrap().as_str() {
                    "Notes" => {
                        // create a Notebook object
                        let provider = gtk::CssProvider::new();
                        provider.load_from_data(
                            "
                            textview {
                                border: 1px solid @borders;
                                border-radius: 4px;
                                padding: 4px;
                            }

                            body {
                                background-color: #2e3440;
                                color: #d8dee9;
                                border-radius: 10px;
                            }
                          
                            "
                        );
                        gtk::StyleContext::add_provider_for_display(
                            &gdk::Display::default().expect("Error initializing GTK CSS provider."),
                            &provider,
                            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                        );
                        let notebook = gtk::Notebook::new();
                        notebook.add_css_class("body");
                        let path_for_data = "/home/yumshot/.config/yum_notes/notes/";
                        let path_for_data = path_for_data.to_string();
                        

                        // loop through the names of the files in path_for_data
                        for entry in std::fs::read_dir(path_for_data).unwrap() {
                            let entry = entry.unwrap();
                            let path = entry.path();
                            let file_name = path.file_name().unwrap().to_str().unwrap();
                            
                            // read the contents of the file and add display it within the notebook 
                            let contents = std::fs::read_to_string(&path).unwrap();
                            let text_view = gtk::TextView::new();
                            let buffer = gtk::TextBuffer::new(None);
                            buffer.set_text( &contents);
                            text_view.set_buffer(Some(&buffer));
                            text_view.set_wrap_mode(gtk::WrapMode::WordChar);
                            text_view.set_accepts_tab(false);
                            text_view.set_height_request(150);
                            text_view.add_css_class("body");
                            notebook.append_page(&text_view, Some(&gtk::Label::new(Some(file_name))));
                        }
                        
                     
                        let new_note = gtk::Box::new(gtk::Orientation::Vertical, 15);
                        new_note.set_halign(gtk::Align::Center);
                        new_note.set_valign(gtk::Align::Center);
                        let new_note_title = gtk::Entry::new();
                        new_note_title.set_placeholder_text(Some("File Name"));
                        let new_note_contents = gtk::TextView::new();
                      
                        new_note_contents.set_wrap_mode(gtk::WrapMode::WordChar);
                        new_note_contents.set_accepts_tab(false);
                        new_note_contents.add_css_class("textview");

                        let tags_possible = ["Feature", "Bug", "Enhancement", "Documentation", "Question", "Other"];
                        let status_possible = ["Todo", "Working", "Done", "Deleted"];

                        // create two drop downs with the possible tags and status
                        let tag_drop_down = gtk::ComboBoxText::new();
                        let status_drop_down = gtk::ComboBoxText::new();
                     

                        for tag in tags_possible.iter() {
                            tag_drop_down.append(Some(tag), tag);
                        }
                        for status in status_possible.iter() {
                            status_drop_down.append(Some(status), status);
                        }

                        status_drop_down.set_active(Some(0));
                        tag_drop_down.set_active(Some(0));

                        new_note.append(&tag_drop_down);
                        new_note.append(&status_drop_down);





                        let scrolled_window = gtk::ScrolledWindow::new();
                        scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
                        scrolled_window.set_min_content_height(50);
                        scrolled_window.set_propagate_natural_height(true);
                        scrolled_window.set_child(Some(&new_note_contents));

                        let new_note_submit = gtk::Button::with_label("Submit");
                        new_note_submit.set_css_classes(&["custom-btn"]);
                        new_note.append(&new_note_title);
                        new_note.append(&scrolled_window);
                        new_note.append(&new_note_submit);
                        
                        notebook.append_page(&new_note, Some(&gtk::Label::new(Some("New Note"))));
                        scrolled_window.add_css_class("body");
                        notebook.add_css_class("body");
                       
                        
                        
                        
                        
                        
                        window_clone.set_child(Some(&notebook));
                    },
                    "Tasks" => println!("Tasks button clicked"),
                    "Github" => println!("Github button clicked"),
                    "Settings" => println!("Settings button clicked"),
                    "Quit" => {
                        println!("Quit button clicked");
                        window_clone.close();
                    },
                    _ => println!("Button clicked"),
                }
            });

        }

        



        window.set_child(Some(&button_box));


         
        window.present();
    });

    application.run()
}


