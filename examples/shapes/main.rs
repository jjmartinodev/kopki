use kopki::{
    graphics::{shape::{Shape, ShapeRenderer}, Frame}, App, AppState
};

struct MyState {
    shape_renderer: ShapeRenderer
}

impl AppState for MyState {
    fn start(app: &mut App) -> Self {
        Self {
            shape_renderer: ShapeRenderer::new(app)
        }
    }
    fn uptade(&mut self, app: &mut App, mut frame: Frame) {
        frame.clear(1.0, 0.0, 1.0, 1.0);
        self.shape_renderer.render(app, &mut frame, &[
            &Shape::Rect { x: 0.0, y: 0.0, w: 200.0, h: 200.0, color: [0,255,255,255] },
            &Shape::Circle { x: 300.0, y: 100.0, r: 100.0, color: [0,255,0,255] }
        ]);
        frame.present(app);
    }
}

fn main() {
    let app = App::new();
    app.run::<MyState>();
}