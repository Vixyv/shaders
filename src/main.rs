#[allow(unused_variables)]

pub mod algorithms;

fn main() {
    setup();

    println!("{}", algorithms::perlin_noise_2d::noise(0.0, 1.0));
    println!("{}", algorithms::simplex_noise_2d::noise(0.0, 1.0));
}
fn setup() {
    #[allow(clippy::unwrap_used)]
    color_eyre::install().unwrap();
}
