use orbtk::prelude::*;

use orbtk::{Application, Window};

#[derive(Copy, Clone)]
enum ProgressEvent {
    Advance(f64),
    Reset,
    SetToFull,
}

#[derive(Default, AsAny)]
struct MainViewState {
    action: Option<ProgressEvent>,
}

widget!(MainView<MainViewState>);

impl MainViewState {
    fn action(&mut self, action: impl Into<Option<ProgressEvent>>) {
        self.action = action.into();
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
           
            self.action = None;
        }
    }
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .build(ctx),
        )
    }
}


fn main() {
    Application::new()
    .window(|ctx| {
        Window::new()
            .title("Main window")
            .position((100.0, 100.0))
            .size(420.0, 420.0)
            .child(MainView::new().build(ctx))
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
    
    // Main Loop
    while true {
        println!("Main Loop");
    }
}
