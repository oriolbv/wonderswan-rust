use orbtk::prelude::*;

use orbtk::{Application, Window};

fn main() {
    Application::new()
    .window(|ctx| {
        Window::new()
            .title("Main window")
            .position((100.0, 100.0))
            .size(420.0, 420.0)
            .build(ctx)
    })
    .run();


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
    
}