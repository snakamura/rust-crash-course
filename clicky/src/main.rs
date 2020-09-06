extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use std::cell::RefCell;
use std::error::Error;
use std::io::Write;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn Error>> {
    let application = Application::new(Some("org.snak.Clicky"), Default::default())?;

    let file = std::fs::File::create("log.txt")?;
    let file = Rc::new(RefCell::new(file));

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Clicky");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click!");
        let file = file.clone();
        button.connect_clicked(
            move |_| match file.borrow_mut().write_all(b"I was clicked.\n") {
                Ok(()) => (),
                Err(e) => eprintln!("Error writing to file: {}", e),
            },
        );
        window.add(&button);

        window.show_all();
    });

    application.run(&[]);

    Ok(())
}
