#![allow(dead_code)]
#![feature(vec_remove_item)]

use std::io::Cursor;
use svg;

use usvg::{FitTo, Options, Tree};

use resvg::render;

use std::io::Write;

use std::io::Result;

use std::process;
mod scene {
    use std::io::Write;
    use svg::Document;

    const FPS: f32 = 25.;
    #[derive(Clone, Debug, PartialEq)]
    pub enum MObj {
        Rectangle(f32, f32, f32, f32, String),
    }
    impl MObj {
        fn to_addable(&self) -> svg::node::element::SVG {
            match self {
                MObj::Rectangle(x, y, w, h, color) => svg::node::element::SVG::new().add(
                    svg::node::element::Rectangle::new()
                        .set("x", x.clone())
                        .set("y", y.clone())
                        .set("width", w.clone())
                        .set("height", h.clone())
                        .set("stroke-width", 3)
                        .set("stroke", color.clone()),
                ),
                #[allow(unreachable_patterns)]
                _ => panic!("help"),
            }
        }
    }

    #[derive(Debug)]
    enum DrawCall<'a> {
        Wait(f32),
        Appear(&'a MObj),
        Disappear(&'a MObj),
    }
    #[derive(Debug)]
    pub struct Scene<'a> {
        draw_calls: Vec<DrawCall<'a>>,
        objects: Vec<&'a MObj>,
        width: i32,
        height: i32,
    }

    impl<'a> Scene<'a> {
        pub fn new(width: i32, height: i32) -> Scene<'a> {
            Scene {
                width,
                height,
                draw_calls: Vec::new(),
                objects: Vec::new(),
            }
        }
        fn add_draw_call(mut self, draw_call: DrawCall<'a>) -> Self {
            self.draw_calls.push(draw_call);
            self
        }

        pub fn wait(self, amt: f32) -> Self {
            self.add_draw_call(DrawCall::Wait(amt))
        }

        pub fn appear(self, mobj: &'a MObj) -> Self {
            self.add_draw_call(DrawCall::Appear(mobj))
        }
        pub fn disappear(self, mobj: &'a MObj) -> Self {
            self.add_draw_call(DrawCall::Disappear(mobj))
        }

        pub fn render(mut self, mut sink: impl Write) -> std::io::Result<()> {
            use crate::save_to_writable;
            for call in self.draw_calls {
                println!("Rendering : {:?}", call);
                match call {
                    DrawCall::Wait(amt) => {
                        let mut svg_image =
                            Document::new().set("viewBox", (0, 0, self.width, self.height));
                        for obj in &self.objects {
                            svg_image = svg_image.add(obj.to_addable());
                        }
                        for _i in 0..(amt * FPS) as i32 {
                            save_to_writable(&svg_image, &mut sink)?;
                        }
                    }
                    DrawCall::Appear(a) => self.objects.push(a),
                    DrawCall::Disappear(a) => {
                        let index = self.objects.iter().position(|x| *x == a).unwrap();
                        self.objects.remove(index);
                    }
                }
            }
            Ok(())
        }
    }
}

fn main() {
    let obj = scene::MObj::Rectangle(250., 250., 100., 100., String::from("blue"));
    let obj2 = scene::MObj::Rectangle(350., 350., 100., 100., String::from("green"));
    let scene = scene::Scene::new(500, 500)
        .wait(1.)
        .appear(&obj)
        .wait(1.)
        .appear(&obj2)
        .wait(1.)
        .wait(300.)
        .disappear(&obj)
        .wait(1.);

    let ffmpeg_pipe = start_ffmpeg().unwrap();

    scene.render(ffmpeg_pipe).unwrap();
}

fn save_to_png<T>(svg_node: T, file_name: String) -> Result<()>
where
    T: svg::node::Node,
{
    // TODO: Refractor using writable
    let mut svg_data = Cursor::new(Vec::new());
    svg::write(&mut svg_data, &svg_node)?;

    let svg_tree = Tree::from_data(&svg_data.into_inner(), &Options::default()).unwrap();

    let image = render(&svg_tree, FitTo::Original, None).unwrap();
    image.save_png(std::path::Path::new(file_name.as_str()))?;

    Ok(())
}

fn save_to_writable(svg_node: &impl svg::node::Node, sink: impl Write) -> Result<()> {
    let mut svg_data = Cursor::new(Vec::new());
    svg::write(&mut svg_data, svg_node)?;

    let svg_tree = Tree::from_data(&svg_data.into_inner(), &Options::default()).unwrap();

    let image = render(&svg_tree, FitTo::Original, None).unwrap();

    //XXX We needed to encode the data as a png before piping to ffmpeg
    //Modified from resvg save_png
    let mut encoder = png::Encoder::new(sink, image.width(), image.height());
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&image.data())?;

    //XXX Somehow the encoder takes ownership of sink but that doesn't break anything
    //sink.write(image.data())?;
    //sink.flush();
    Ok(())
}

fn start_ffmpeg() -> Option<process::ChildStdin> {
    let ffmpeg = process::Command::new("ffmpeg")
        .stdin(process::Stdio::piped())
        .args(&[
            "-hide_banner", // Quiets
            "-loglevel",    // Silences all messages
            "panic",
            "-y", // Force overwrite
            "-r",
            "25",
            "-i", // Use stdin
            "-",
            "-c:v",
            "libx264",
            "-vf",
            "fps=25",
            "-pix_fmt",
            "yuv420p",
            "final.mp4",
        ])
        .spawn()
        .unwrap();
    ffmpeg.stdin
}
