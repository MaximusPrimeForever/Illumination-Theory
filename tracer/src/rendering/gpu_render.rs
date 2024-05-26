use bytemuck::{Pod, Zeroable};
use wgpu::PipelineCompilationOptions;


#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Uniforms {
    width: u32,
    height: u32,
    pub frame_count: u32
}

pub struct PathTracer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    display_pipeline: wgpu::RenderPipeline,
    display_bind_group: wgpu::BindGroup
}

impl PathTracer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, width: u32, height: u32) -> PathTracer {
        device.on_uncaptured_error(Box::new(|error| {
            panic!("Aborting due to an error: {}", error);
        }));

        let shader_module = compile_shader_module(&device);
        let (display_pipeline, display_layout) = 
            create_display_pipeline(&device, &shader_module);

        let uniforms = Uniforms {
            width,
            height,
            frame_count: 0
        };
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM  | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let display_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: None,
            layout: &display_layout,
            entries: &[wgpu::BindGroupEntry{
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: None
                })
            }]
        });

        PathTracer {
            device,
            queue,
            uniforms,
            uniform_buffer,
            display_pipeline,
            display_bind_group
        }
    }

    pub fn render_frame(&mut self, target: &wgpu::TextureView) {
        self.uniforms.frame_count += 1;
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::bytes_of(&self.uniforms)
        );
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
        render_pass.set_bind_group(
            0, 
            &self.display_bind_group, 
            &[]
        );

        // Draw 1 instance of a polygon with 3 vertices
        render_pass.draw(0..6, 0..1);

        // End render pass by consuming the object
        drop(render_pass);

        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));
    }
}


// Auxilary Functions
// ==================

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
) -> (wgpu::RenderPipeline, wgpu::BindGroupLayout) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }
        ]
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("display"),
        layout: Some(&device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
                ..Default::default()
        })),
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
    });

    return (render_pipeline, bind_group_layout)
}