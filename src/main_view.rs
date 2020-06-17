

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