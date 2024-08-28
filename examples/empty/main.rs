use kopki::RenderInstance;

fn main() {
    let instance = RenderInstance::new();
    let device = instance.device_from_instance();
    _ = device;
}
