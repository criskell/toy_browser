use toy_browser::{css, html, layout::{self, BoxDimensions, Rect}, style, painting};
use image;

fn main() {
    let root = html::parse("<div class=\"a\">
    <div class=\"b\">
        <div class=\"c\">
            <div class=\"d\">
                <div class=\"e\">
                    <div class=\"f\">
                        <div class=\"g\"></div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>".to_owned());

    let stylesheet = css::parse(
        "
* { display: block; padding: 12px; }
.a { background-color: #ff0000ff; }
.b { background-color: #ffa500ff; }
.c { background-color: #ffff00ff; }
.d { background-color: #008000ff; }
.e { background-color: #0000ffff; }
.f { background-color: #4b0082ff; }
.g { background-color: #800080ff; height: 432px; }
"
        .to_owned(),
    );

    let styled = style::style_node(&root, &stylesheet);

    let initial_containing_block = BoxDimensions {
        content: Rect {
            width: 800.0,
            height: 600.0,
            ..Default::default()
        },
        ..Default::default()
    };

    let layout_box = layout::layout_node(&styled, initial_containing_block.clone());

    let canvas = painting::paint_node(&layout_box, initial_containing_block.content.clone());

    let mut buffer = image::ImageBuffer::new(800, 600);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let color = canvas.pixels.get((y * 800 + x) as usize).unwrap();

        *pixel = image::Rgba([color.r, color.g, color.b, color.a]);
    }

    buffer.save("output.png").unwrap();
}