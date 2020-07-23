#![allow(dead_code)]
use std::env;

pub mod mobject;
pub mod render;
pub mod scene;

use crate::mobject::*;
use crate::scene::*;

fn main() {
    let outfile = env::args()
        .nth(1)
        .unwrap_or_else(|| "final.mp4".to_string());
    println!("Writing to {}", outfile);

    let obj = Mobject::Rectangle {
        x: 250.,
        y: 250.,
        w: 100.,
        h: 100.,
        color: String::from("blue"),
    };
    let obj2 = Mobject::Rectangle {
        x: 350.,
        y: 350.,
        w: 100.,
        h: 100.,
        color: String::from("green"),
    };
    let mut scene = Scene::new(500, 500).appear(&obj2).fade_out(&obj2, 5.);

    let ffmpeg_pipe = render::start_ffmpeg(outfile, 500, 500).unwrap();

    scene.render(ffmpeg_pipe).unwrap();
}
