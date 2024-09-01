# kopki

Small engine for creating graphical contexts and framebuffers with winit and wgpu.

# Objectives
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

for more examples look for the examples folder in the github repository.