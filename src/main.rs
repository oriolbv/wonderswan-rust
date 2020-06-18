mod main_view;

extern crate sciter;

fn main() {
    let mut frame = sciter::Window::new();
    frame.load_file("minimal.htm");
    frame.run_app();
}

// fn main() {
    
//     println!("HOLI");
    
//     // main_view::init_main_view();

//     // Init variables
//     // let wsc : i32 = 1;
// 	// let screen_size : i32 = 2;
// 	// let flipd : i32 = 0;
// 	// let wsShades : i32 = 0;
// 	// let wsCycles : i32 = 0;
// 	// let wsLine : i32 = 0;
// 	// let rom_size : i32 = 128000000;
// 	// let wsMakeScr : i32 = 0;
// 	// let fr : i32 = 0;
// 	// let frameskip : i32 = 1;
// 	// let wsVMode : i32 = -1;
//     // let vsync : i32 = 1;
    
//     // Main Loop
//     // loop { 
//     //     println!("Main Loop");
//     // }
// }
