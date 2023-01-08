use std::collections::HashMap;
use toy_browser::dom::Node;

fn main() {
    let example_tree = Node::Element {
        tag_name: "html".to_owned(),
        attributes: HashMap::from([(String::from("lang"), String::from("pt_BR"))]),
        children: vec![Node::Element {
            tag_name: "body".to_owned(),
            attributes: HashMap::new(),
            children: vec![
                Node::Text {
                    value: String::from("Olá, mundo!"),
                },
                Node::Element {
                    tag_name: String::from("p"),
                    attributes: HashMap::new(),
                    children: vec![Node::Text {
                        value: String::from("Parágrafo."),
                    }],
                },
                Node::Text {
                    value: String::from("teste"),
                },
            ],
        }],
    };

    println!("{}", example_tree.markup());
}
