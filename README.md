# kopki

Simple graphics engine, made with winit for windowing & user input, and wgpu-rs for graphics.

# Objectives
- Manage rendering in high levels of abstraction layers, or use wgpu with the engine's context.
- Be easy enough to prototype things moderatly fast.
- Windows and Linux support.

# Non-Objectives
- Android and MacOs support.

# Minimal Example
```
use kopki::{
    graphics::Frame, App, AppState
};

struct State;

impl AppState for State {
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
    app.run::<State>();
}
```

for more examples look for the examples folder in the github repo