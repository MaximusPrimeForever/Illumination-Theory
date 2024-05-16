
pub struct PathTracer {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl PathTracer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> PathTracer {
        device.on_uncaptured_error(Box::new(|error| {
            panic!("Aborting due to an error: {}", error);
        }));

        let shader_module = compile_shader_module(&device);
        // TODO: initialize GPU resources

        PathTracer {
            device,
            queue,
        }
    }
}

fn compile_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    use std::borrow::Cow;

    let shader_code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "\\..\\shaders\\shaders.wgsl"));
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_code)),
    })
}