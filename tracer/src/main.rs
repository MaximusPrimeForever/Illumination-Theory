mod math;
mod graphics;
mod geometry;
mod rendering;
mod scenes;

use std::env;
use std::sync::Arc;

use graphics::Camera;
use rendering::{gpu_render, render::render_scene};
// use buffer::write_img_ppm;

use math::vec3::{Vec3, Point3, Color};
use winit::window::Window;
use {
    anyhow::{Context, Result},
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
};

// Assign the appropriate window size in terms of physical pixels based on your display DPI.
const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;


fn main_fake() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        scenes::test_scene();
        return Ok(());
    }
    if args.len() != 7 {
        panic!("Invalid arguments");
    }

    let image_width = args[1].parse::<usize>().unwrap();
    let aspect_ratio = args[2].parse::<f64>().unwrap();
    let vfov = args[3].parse::<f64>().unwrap();
    let samples_per_pixel: usize = args[4].parse::<usize>().unwrap();
    let trace_depth: usize = args[5].parse::<usize>().unwrap();
    let core_count: usize = args[6].parse::<usize>().unwrap();

    // debug - because I didn't bother to check how to pass arguments to the
    // vscode rust debugger
    // let image_width = 400;
    // let aspect_ratio = 16.0 / 9.0;
    // let vfov: f64 = 20.0;
    // let samples_per_pixel: usize = 50;
    // let trace_depth: usize = 10;
    // let core_count: usize = 1;

    // Control some parameters of camera from CLI
    let mut cam = Camera::default();
    cam.image_width = image_width;
    cam.image_height = (image_width as f64 / aspect_ratio) as usize;
    cam.vfov = vfov;
    
    // World
    // World functions configure camera position, orientation, focus, etc.
    // let world = rtweekend::one_weekend_endgame(8);
    // let world = rtweekend::cool_effects(8, 1.3);
    // let world = rtweekend::lit_world_textures(&mut cam);
    // let world = rtweekend::two_checkered_spheres(&mut cam);
    // let world = rtweekend::earth(&mut cam);
    // let world = rtweekend::tiled_texture(&mut cam);
    // let world = rtweekend::quad_scene(&mut cam);
    // let world = rtweekend::sphereflake_on_sandy_plane(&mut cam);
    // let world = rtweekend::quad_shadow_test(&mut cam);
    // let world = rtweekend::cornell_box(&mut cam);
    // let world = scenes::cornell_box_dark_sphereflake(&mut cam);
    // let world = scenes::cornell_box_with_smokey_boxes(&mut cam);
    let world = scenes::book2_final_scene(&mut cam);
    // let world = rtweekend::lit_world(&mut cam);

    // Must be called!
    cam.initialize();
    
    // Render
    let image_canvas = render_scene(
        core_count,
        world,
        Arc::new(cam),
        samples_per_pixel,
        trace_depth
    );
    
    // Output to file
    image_canvas.save_png("output.png");

    Ok(())
}

#[pollster::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new().unwrap();
    let window_size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_resizable(true)
        .with_title("GPU Path Tracer".to_string())
        .build(&event_loop)?;
    let (device, queue, surface) = connect_to_gpu(&window).await?;
    let renderer = gpu_render::PathTracer::new(device, queue);

    event_loop.run(|event, control_handle| {
        control_handle.set_control_flow(ControlFlow::Poll);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_handle.exit(),
                WindowEvent::RedrawRequested => {
                    let frame: wgpu::SurfaceTexture = surface
                        .get_current_texture()
                        .expect("failed to get current texture");

                    // TODO: draw frame

                    frame.present();
                    window.request_redraw();
                }
                _ => (),
            },
            _ => (),
        }
    })?;
    Ok(())
}

async fn connect_to_gpu(window: &Window) -> Result<(wgpu::Device, wgpu::Queue, wgpu::Surface)> {
    use wgpu::TextureFormat::{Bgra8Unorm, Rgba8Unorm};

    // Create an "instance" of wgpu. This is the entry-point to the API.
    let instance = wgpu::Instance::default();

    // Create a drawable "surface" that is associated with the window.
    let surface = instance.create_surface(window)?;

    // Request a GPU that is compatible with the surface. If the system has multiple GPUs then
    // pick the high performance one.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .context("failed to find a compatible adapter")?;

    // Connect to the GPU. "device" represents the connection to the GPU and allows us to create
    // resources like buffers, textures, and pipelines. "queue" represents the command queue that
    // we use to submit commands to the GPU.
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .context("failed to connect to the GPU")?;

    // Configure the texture memory backs the surface. 
    // Our renderer will draw to a surface texture every frame.
    let caps = surface.get_capabilities(&adapter);
    let format = caps
        .formats
        .into_iter()
        .find(|it| matches!(it, Rgba8Unorm | Bgra8Unorm))
        .context("could not find preferred texture format (Rgba8Unorm or Bgra8Unorm)")?;
    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 3,
    };
    surface.configure(&device, &config);

    Ok((device, queue, surface))
}