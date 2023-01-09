use toy_browser::css;

fn main() {
    let stylesheet = css::parse("

* {
    box-sizing: border-box;
    margin: 0px;
    padding: 0px;
}

body {
    font-family: Arial;
    font-size: 16px;
}

#foo, abc.bar.k#d, *.f.g.h#k, bfc.#a4fa {
    font-size: 16px;
    font-family: Arial;
}

".to_owned());

    println!("{:#?}", stylesheet);
}
