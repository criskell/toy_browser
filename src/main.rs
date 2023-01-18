use toy_browser::{css, html, layout::{self, BoxDimensions, Rect}, style};

fn main() {
    let root = html::parse("<a><b></b></a>".to_owned());

    let stylesheet = css::parse(
        "
a {
    width: 50px;
    display: block;
}

b {
    height: 30px;
    display: block;
}
"
        .to_owned(),
    );

    let styled = style::style_node(&root, &stylesheet);

    let layout_box = layout::layout_node(&styled, BoxDimensions {
        content: Rect {
            width: 100.0,
            height: 50.0,
            ..Default::default()
        },
        ..Default::default()
    });

    println!("{:#?}", layout_box);
}