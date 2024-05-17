use wgpu::PipelineCompilationOptions;


pub struct PathTracer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    display_pipeline: wgpu::RenderPipeline
}

impl PathTracer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> PathTracer {
        device.on_uncaptured_error(Box::new(|error| {
            panic!("Aborting due to an error: {}", error);
        }));

        let shader_module = compile_shader_module(&device);
        let display_pipeline = create_display_pipeline(&device, &shader_module);

        PathTracer {
            device,
            queue,
            display_pipeline
        }
    }

    pub fn render_frame(&self, target: &wgpu::TextureView) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render frame"),
            });
        
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("display pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations { 
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store
                },
            })],
            ..Default::default()
        });
        render_pass.set_pipeline(&self.display_pipeline);

        // Draw 1 instance of a polygon with 3 vertices
        render_pass.draw(0..3, 0..1);

        // End render pass by consuming the object
        drop(render_pass);

        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));
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

fn create_display_pipeline(
    device: &wgpu::Device,
    shadler_module: &wgpu::ShaderModule
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("display"),
        layout: None,
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        },
        vertex: wgpu::VertexState {
            module: shadler_module,
            entry_point: "display_vs",
            buffers: &[],
            compilation_options: PipelineCompilationOptions::default() // since 0.20.0, default is fine for most cases
        },
        fragment: Some(wgpu::FragmentState {
             module: shadler_module,
             entry_point: "display_fs",
             compilation_options:  PipelineCompilationOptions::default(), // since 0.20.0, default is fine for most cases
             targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8Unorm,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
             })]
        }),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None
    })
}