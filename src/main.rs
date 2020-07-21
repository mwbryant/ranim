#![allow(dead_code)]
use std::env;

pub mod mobject;
pub mod render;
pub mod scene;

use crate::mobject::*;
use crate::render::*;
use crate::scene::*;

fn main() {
    let outfile = env::args().nth(1).unwrap_or("final.mp4".to_string());
    println!("Writing to {}", outfile);

    let obj = Mobject::Rectangle(250., 250., 100., 100., String::from("blue"));
    let obj2 = Mobject::Rectangle(350., 350., 100., 100., String::from("green"));
    let scene = Scene::new(500, 500)
        .wait(1.)
        .appear(&obj)
        .wait(1.)
        .appear(&obj2)
        .wait(1.)
        .disappear(&obj)
        .wait(1.);

    let ffmpeg_pipe = render::start_ffmpeg(outfile).unwrap();

    scene.render(ffmpeg_pipe).unwrap();
}
