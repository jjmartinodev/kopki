use kopki::{
    graphics::Frame, App, AppState
};

struct MyState;

impl AppState for MyState {
    fn start(_app: &mut App) -> Self {
        Self
    }
    fn uptade(&mut self, app: &mut App, mut frame: Frame) {
        frame.clear(1.0, 0.0, 1.0, 1.0);
        frame.present(app);
    }
}

fn main() {
    let app = App::new();
    app.run::<MyState>();
}