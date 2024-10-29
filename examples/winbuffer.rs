#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use jdrift_interface::{renderer::Renderer, vec2, Window};
use ocl::{Device, Platform};
use jdrift_interface::renderer::shape::{flatten_curve, Segment, Shape};
use jdrift_interface::renderer::vec2::{Ndc, Vec2};

const SIZE: [u64; 2] = [400, 200];

fn main() {
    // let device = Device::first(Platform::first().unwrap()).unwrap();
    // let mut window = Window::new(Renderer::new(device, SIZE).expect("Window size is too large"));
    // window.start();

    let circle = Shape {
        points: vec![
            Segment::Curve { points: [ [0, 10], [0, 5], [5, 0], [10, 0] ] }
        ]
    };

    // dbg!(flatten_curve([ vec2!(-1.0, -1.0), vec2!(0.0, 0.0), vec2!(0.0, 1.0), vec2!(1.0, 1.0) ], 5));
    dbg!(vec2!(-1.0, -1.0).slope(vec2!(1.0, 0.8)));
}