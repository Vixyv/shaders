#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::prelude::*;

pub mod algorithms;
pub mod prelude;

fn main() {
    setup();

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

fn setup() {
    #[allow(clippy::unwrap_used)]
    color_eyre::install().unwrap();
}
