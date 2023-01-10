use toy_browser::{css, html, style};

fn main() {
    let root = html::parse(
        "<html>
    <head>
        <title>EU SEIIIIIIIIIIIIIII</title>
    </head>
    <body>
        <p id=\"paragraph\" class=\"ff aah bbbb\">OLA A</p>
    </body>
</html>"
        .to_owned(),
    );

    let stylesheet = css::parse(
        "
head, title {
    display: none;
}

body {
    width: 500px;
    height: 500px;
    display: block;
}

p#paragraph.aah.bbbb {
    propriedade: aaaaaaaaaaaaaaaaaaaaaa;
}

#paragraph.aah.bbbb {
    propriedade: OLA;
}
"
        .to_owned(),
    );

    let styled = style::style_node(&root, &stylesheet);

    println!("{:#?}", styled);
}
