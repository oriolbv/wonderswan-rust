mod main_view;

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, Button, MessageDialog};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("builder_basics.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    let bigbutton: Button = builder.get_object("button1").expect("Couldn't get button1");
    let dialog: MessageDialog = builder
        .get_object("messagedialog1")
        .expect("Couldn't get messagedialog1");

    bigbutton.connect_clicked(move |_| {
        dialog.run();
        dialog.hide();
    });

    window.show_all();
}

fn main() {
    
    println!("HOLI");
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());

    // main_view::init_main_view();

    // Init variables
    // let wsc : i32 = 1;
	// let screen_size : i32 = 2;
	// let flipd : i32 = 0;
	// let wsShades : i32 = 0;
	// let wsCycles : i32 = 0;
	// let wsLine : i32 = 0;
	// let rom_size : i32 = 128000000;
	// let wsMakeScr : i32 = 0;
	// let fr : i32 = 0;
	// let frameskip : i32 = 1;
	// let wsVMode : i32 = -1;
    // let vsync : i32 = 1;
    
    // Main Loop
    // loop { 
    //     println!("Main Loop");
    // }
}
