use toy_browser::html;

fn main() {
    let tree = html::parse(String::from("<html>
    <body>
        <h1>Title</h1>
        <div id=\"main\" class=\"test\">
            <p>Hello <em>world</em>!</p>
        </div>
    </body>
</html>
"));

    println!("{:#?}", tree);
}
