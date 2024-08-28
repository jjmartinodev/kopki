# kopki

Versatile graphics engine, made with winit for windowing, and wgpu-rs for graphics.

# Objectives
- Manage rendering in high levels of abstraction layers, or directly use wgpu with the engine's supplied context.
- Windows and Linux support.

# Non-Objectives
- Android and MacOs support.

# Minimal Example
```
use kopki::RenderInstance;

fn main() {
    let instance = RenderInstance::new();
    let device = instance.device_from_instance();
    _ = device;
}
```

for more examples look for the examples folder in the github repo