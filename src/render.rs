use svg;

use resvg::render;
use std::io::{Cursor, Result, Write};
use std::path::Path;
use std::process;
use std::process::{Command, Stdio};
use usvg::{FitTo, Options, Tree};

pub fn save_to_png<T>(svg_node: T, file_name: String) -> Result<()>
where
    T: svg::node::Node,
{
    // TODO: Refractor using writable
    let mut svg_data = Cursor::new(Vec::new());
    svg::write(&mut svg_data, &svg_node)?;

    let svg_tree = Tree::from_data(&svg_data.into_inner(), &Options::default()).unwrap();

    let image = render(&svg_tree, FitTo::Original, None).unwrap();
    image.save_png(Path::new(file_name.as_str()))?;

    Ok(())
}

pub fn save_to_writable(svg_node: &impl svg::node::Node, sink: impl Write) -> Result<()> {
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

pub fn start_ffmpeg(outfile: String) -> Option<process::ChildStdin> {
    let ffmpeg = Command::new("ffmpeg")
        .stdin(Stdio::piped())
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
            &outfile,
        ])
        .spawn()
        .unwrap();
    ffmpeg.stdin
}
