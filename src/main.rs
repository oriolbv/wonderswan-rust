mod main_view;

use std::thread;
use std::time::Duration;

extern crate sciter;

fn main() {
    let handle = thread::spawn(|| {
        // Init variables
        let wsc : i32 = 1;
        let screen_size : i32 = 2;
        let flipd : i32 = 0;
        let wsShades : i32 = 0;
        let wsCycles : i32 = 0;
        let wsLine : i32 = 0;
        let rom_size : i32 = 128000000;
        let wsMakeScr : i32 = 0;
        let fr : i32 = 0;
        let frameskip : i32 = 1;
        let wsVMode : i32 = -1;
        let vsync : i32 = 1;
        
        loop {
            println!("Main Loop");
        }
    });

	let html = include_bytes!("minimal.htm");

	sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8		// Enables `Sciter.machineName()`
		| sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8	// Enables opening file dialog (`view.selectFile()`)
		)).unwrap();

	sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).unwrap();

	let mut frame = sciter::Window::new();

    frame.set_options(sciter::window::Options::DebugMode(true)).unwrap();

	frame.load_html(html, Some("example://minimal.htm"));

    frame.run_app();

}


// fn main() {
    
//     
    
//     // main_view::init_main_view();


    

