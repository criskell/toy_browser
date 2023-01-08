use std::collections::HashMap;

#[derive(Debug)]
pub enum Node {
    Text {
        value: String,
    },
    Element {
        tag_name: String,
        attributes: AttrMap,
        children: Vec<Node>,
    },
}

pub type AttrMap = HashMap<String, String>;

impl Node {
    pub fn markup(&self) -> String {
        get_node_markup(self, 0)
    }
}

fn get_node_markup(node: &Node, current_depth: i32) -> String {
    let ident = "    ".repeat(current_depth.try_into().unwrap());

    match node {
        Node::Text { value } => ident + value,
        Node::Element {
            tag_name,
            attributes,
            children,
        } => {
            let attributes_markup = attributes
                .iter()
                .map(|(name, value)| format!("{}=\"{}\"", name, value))
                .collect::<Vec<String>>()
                .join(" ");
            let attributes_markup = if attributes_markup.is_empty() {
                "".to_owned()
            } else {
                " ".to_owned() + &attributes_markup
            };

            let content_markup = children
                .iter()
                .map(|node| get_node_markup(node, current_depth + 1))
                .collect::<Vec<String>>()
                .join("\n");

            format!(
                "{ident}<{name}{attrs}>\n{content}\n{ident}</{name}>",
                name = tag_name,
                attrs = attributes_markup,
                content = content_markup,
            )
        }
    }
}
