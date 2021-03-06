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
    AnimateProperty {
        object: &'a Mobject,
        property_name: String,
        starting_value: f32,
        ending_value: f32,
        duration: f32,
    },
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

    pub fn fade_out(self, mobj: &'a Mobject, duration: f32) -> Self {
        self.add_draw_call(DrawCall::AnimateProperty {
            object: mobj,
            property_name: String::from("opacity"),
            starting_value: 1.,
            ending_value: 0.,
            duration,
        })
    }

    pub fn render(&mut self, mut sink: impl Write) -> std::io::Result<()> {
        use crate::render::save_to_writable;
        for call in &self.draw_calls {
            //println!("Rendering : {:?}", call);
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
                    self.objects.retain(|x| x != a);
                }
                DrawCall::AnimateProperty {
                    object,
                    property_name,
                    starting_value,
                    ending_value,
                    duration,
                } => {
                    let ending_frame = duration * FPS;
                    for i in 0..ending_frame as i32 {
                        // Create the current value from the current frame number
                        let current_value = ((i as f32 / ending_frame as f32)
                            * (ending_value - starting_value) as f32)
                            + *starting_value as f32;

                        let mut svg_image =
                            Document::new().set("viewBox", (0, 0, self.width, self.height));

                        for obj in &self.objects {
                            let final_object = if obj == object {
                                obj.to_addable().set(property_name, current_value)
                            } else {
                                obj.to_addable()
                            };
                            svg_image = svg_image.add(final_object);
                        }
                        save_to_writable(&svg_image, &mut sink)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[test]
fn appear_test() {
    let obj = Mobject::Rectangle {
        x: 250.,
        y: 250.,
        w: 100.,
        h: 100.,
        color: String::from("blue"),
    };

    let mut scene = Scene::new(500, 500).appear(&obj);
    scene
        .render(std::io::sink())
        .expect("Problem throwing away render");
    assert_eq!(scene.objects.len(), 1);
}

#[test]
fn appear_then_disappear_test() {
    let obj = Mobject::Rectangle {
        x: 250.,
        y: 250.,
        w: 100.,
        h: 100.,
        color: String::from("blue"),
    };

    let mut scene = Scene::new(500, 500).appear(&obj).disappear(&obj);
    scene
        .render(std::io::sink())
        .expect("Problem throwing away render");
    assert_eq!(scene.objects.len(), 0);
}

#[test]
fn many_appear_then_disappear_test() {
    let mut scene = Scene::new(500, 500);
    let mut objects = Vec::new();

    for i in 0..25 {
        let obj = Mobject::Rectangle {
            x: i as f32,
            y: 250.,
            w: 100.,
            h: 100.,
            color: String::from("blue"),
        };
        objects.push(obj);
    }

    for i in &objects {
        scene = scene.appear(&i);
    }

    scene = scene.wait(0.03);
    scene = scene.disappear(&objects.last().unwrap());
    scene
        .render(std::io::sink())
        .expect("Problem throwing away render");

    assert_eq!(scene.objects.len(), 24);
}

//TODO test appearings objects in random order and make sure all are visible
