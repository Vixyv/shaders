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

pub fn run() {
    env_logger::init();
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
                            physical_key: PhysicalKey::Code(Escape),
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
