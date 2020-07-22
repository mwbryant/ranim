#[derive(Clone, Debug, PartialEq)]
pub enum Mobject {
    Rectangle {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: String,
    },
    Circle(f32, f32, f32, String),
}

impl Mobject {
    pub fn to_addable(&self) -> svg::node::element::SVG {
        match self {
            Mobject::Rectangle { x, y, w, h, color } => svg::node::element::SVG::new().add(
                svg::node::element::Rectangle::new()
                    .set("x", x.clone())
                    .set("y", y.clone())
                    .set("width", w.clone())
                    .set("height", h.clone())
                    .set("stroke-width", 3)
                    .set("stroke", color.clone()),
            ),
            #[allow(unreachable_patterns)]
            _ => panic!(""),
        }
    }
}
