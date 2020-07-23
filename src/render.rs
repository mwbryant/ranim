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

pub fn save_to_writable(svg_node: &impl svg::node::Node, sink: &mut impl Write) -> Result<()> {
    let mut svg_data = Cursor::new(Vec::new());
    svg::write(&mut svg_data, svg_node)?;

    let svg_tree = Tree::from_data(&svg_data.into_inner(), &Options::default()).unwrap();

    let image = render(&svg_tree, FitTo::Original, None).unwrap();

    sink.write_all(image.data())?;
    Ok(())
}

pub fn start_ffmpeg(outfile: String, width: i32, height: i32) -> Option<process::ChildStdin> {
    let ffmpeg = Command::new("ffmpeg")
        .stdin(Stdio::piped())
        .args(&[
            "-hide_banner", // Quiets
            "-loglevel",    // Silences all messages
            "panic",
            "-y", // Force overwrite
            "-r",
            "25",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgba", // Set the input format as RGBA
            "-s",
            format!("{}x{}", width, height).as_str(), // Set the image size
            "-i",                                     // Use stdin
            "-",
            "-c:v",
            "libx264rgb",
            "-vf",
            "fps=25",
            "-pix_fmt",
            "yuv444p",
            &outfile,
        ])
        .spawn()
        .unwrap();
    ffmpeg.stdin
}
