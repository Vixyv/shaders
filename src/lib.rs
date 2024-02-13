#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod algorithms;
pub mod prelude;

use std::char::EscapeUnicode;
use std::str::EscapeDefault;

use winit::event::WindowEvent::KeyboardInput;
use winit::keyboard::PhysicalKey;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};



use winit::window::Window;

struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,
}

impl State {
    // Creating some of the wgpu types requires async code
    // SHOULD RETURN `SELF`
    async fn new(window: Window) -> () {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let surface = instance.create_surface(&window).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}




pub fn run() {
    setup();
    
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => { control_flow.exit(); }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(escape),
                            .. 
                        },
                    ..
                } => { control_flow.exit(); }
                _ => {}
            },
            _ => {}
        }
    }).unwrap();
}

fn setup() {
    #[allow(clippy::unwrap_used)]
    color_eyre::install().unwrap();
    env_logger::init();
}

// Only Temporary
pub fn calc_speed() {
    // Code to calculate the speed of functions
    let mut code_speed_results = vec![];

    for n in 1..1000 {
        use std::time::Instant;
        let now = Instant::now();

        let perlin_noise = crate::algorithms::noise_3d::perlin_noise::PerlinNoise3D::new();

        // Code block to measure.
        {
            for n in 1..100 {
                perlin_noise.noise(1.0, 1.0, 1.0);
            }
        }

        let elapsed = now.elapsed();
        code_speed_results.push(elapsed.as_nanos());
    }

    let avg: f32;

    let mut sum: u128 = 0;
    for x in &code_speed_results {
        sum = sum + x;
    }

    avg = sum as f32 / code_speed_results.len() as f32;

    println!("{}", avg / 100000.0);
}
