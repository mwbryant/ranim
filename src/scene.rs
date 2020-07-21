//use std::collections::BTreeSet;
use std::io::Write;
use svg::Document;

use crate::mobject::*;

const FPS: f32 = 25.;

#[derive(Debug)]
enum DrawCall<'a> {
    Wait(f32),
    Appear(&'a Mobject),
    Disappear(&'a Mobject),
}
#[derive(Debug)]
pub struct Scene<'a> {
    draw_calls: Vec<DrawCall<'a>>,
    objects: Vec<&'a Mobject>,
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

    pub fn appear(self, mobj: &'a Mobject) -> Self {
        self.add_draw_call(DrawCall::Appear(mobj))
    }
    pub fn disappear(self, mobj: &'a Mobject) -> Self {
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
                    self.objects.retain(|x| *x != a);
                }
            }
        }
        Ok(())
    }
}
