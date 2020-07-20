#![allow(dead_code)]
use std::io::Cursor;
use svg;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

use usvg::{FitTo, Options, Tree};

use resvg::render;

use std::io::Write;

use std::io::Result;

use std::process;
//mod scene {
//    pub trait DrawCall {
//
//        fn draw_call
//    }
//    pub struct Scene {
//        duration: f32,
//        current_duration: f32,
//        draw_calls: Vec<DrawCall>,
//    }
//    impl Scene {
//        fn new()
//
//
//    }
//}

fn main() {
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "green")
        .set("stroke-width", 3)
        .set("d", data);

    let data2 = Data::new()
        .move_to((-4, 10))
        .line_by((0, 20))
        .line_by((20, 0))
        .line_by((40, 50));

    let path2 = Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 3)
        .set("d", data2);

    let document = Document::new().set("viewBox", (0, 0, 70, 70)).add(path);
    let document2 = Document::new().set("viewBox", (0, 0, 70, 70)).add(path2);

    //let (head_pipe, tail_pipe) = UnixStream::pair().unwrap();
    //let tail_pipe = tail_pipe.into_raw_fd();
    //let infile = unsafe { std::fs::File::from_raw_fd(tail_pipe) };

    let mut ffmpeg_pipe = start_ffmpeg().unwrap();

    save_to_writable(document, &mut ffmpeg_pipe).unwrap();
    save_to_writable(document2, &mut ffmpeg_pipe).unwrap();

    //save_to_png(document, String::from("image1.png")).unwrap();
    //save_to_png(document2, String::from("image2.png")).unwrap();
}

fn save_to_png<T>(svg_node: T, file_name: String) -> Result<()>
where
    T: svg::node::Node,
{
    let mut svg_data = Cursor::new(Vec::new());
    svg::write(&mut svg_data, &svg_node)?;

    let svg_tree = Tree::from_data(&svg_data.into_inner(), &Options::default()).unwrap();

    let image = render(&svg_tree, FitTo::Original, None).unwrap();
    image.save_png(std::path::Path::new(file_name.as_str()))?;

    Ok(())
}

fn save_to_writable(svg_node: impl svg::node::Node, sink: &mut impl Write) -> Result<()> {
    let mut svg_data = Cursor::new(Vec::new());
    svg::write(&mut svg_data, &svg_node)?;

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
            "1/5",
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
