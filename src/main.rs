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
}