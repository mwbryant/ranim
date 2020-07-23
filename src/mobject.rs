use svg::node::element;

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
    pub fn to_addable(&self) -> element::SVG {
        match self {
            Mobject::Rectangle { x, y, w, h, color } => svg::node::element::SVG::new().add(
                svg::node::element::Rectangle::new()
                    .set("x", *x)
                    .set("y", *y)
                    .set("width", *w)
                    .set("height", *h)
                    .set("stroke-width", 3)
                    .set("stroke", color.clone()),
            ),
            #[allow(unreachable_patterns)]
            _ => panic!(""),
        }
    }
}

#[test]
fn rectangle_to_svg_test() {
    let rect = Mobject::Rectangle {
        x: 0.0,
        y: 150.5,
        w: 50.0,
        h: 50.0,
        color: String::from("blue"),
    };

    let svg_string = rect.to_addable().to_string();
    println!("{}", svg_string);
    let svg_components = svg_string
        .split(&[' ', '\n', '/', '<', '>'][..])
        .collect::<Vec<&str>>();

    assert_eq!(
        svg_components
            .iter()
            .find(|a| a.contains("height"))
            .expect("Rect did not contain a height"),
        &"height=\"50\""
    );
    assert_eq!(
        svg_components
            .iter()
            .find(|a| a.contains("x="))
            .expect("Rect did not contain an x"),
        &"x=\"0\""
    );
    assert_eq!(
        svg_components
            .iter()
            .find(|a| a.contains("y="))
            .expect("Rect did not contain a y"),
        &"y=\"150.5\""
    );
    assert_eq!(
        svg_components
            .iter()
            .find(|a| a.contains("stroke="))
            .expect("Rect did not contain a color"),
        &"stroke=\"blue\""
    );
}
