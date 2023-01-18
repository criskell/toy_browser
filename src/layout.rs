use crate::{style::StyledNode, css::CSSValue};

#[derive(Debug, Default)]
pub struct BoxDimensions {
    content: Rect,
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

#[derive(Debug, Default)]
pub struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

#[derive(Debug, Default)]
pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug)]
pub enum BoxType<'a> {
    Inline(&'a StyledNode<'a>),
    Block(&'a StyledNode<'a>),
    AnonymousBlock,
}

#[derive(Debug)]
pub struct LayoutBox<'a> {
    pub dimensions: BoxDimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

pub fn layout_node<'a>(styled_node: &'a StyledNode) -> LayoutBox<'a> {
    let mut layout_box = LayoutBox {
        box_type: match styled_node.specified_properties.get("display") {
            Some(CSSValue::Keyword(ref s)) if s == "block" => BoxType::Block(styled_node),
            Some(CSSValue::Keyword(ref s)) if s == "none" => panic!("Não é possível construir uma layout box para um nó raiz que tem display: none"),
            _ => BoxType::Inline(styled_node),
        },
        dimensions: Default::default(),
        children: vec![]
    };

    for child in &styled_node.children {
        match child.specified_properties.get("display") {
            Some(CSSValue::Keyword(ref s)) if s == "block" => {
                layout_box.children.push(layout_node(child))
            },
            Some(CSSValue::Keyword(ref s)) if s == "none" => {},
            _ => {
                let mut anonymous_box = LayoutBox {
                    dimensions: Default::default(),
                    box_type: BoxType::AnonymousBlock,
                    children: vec![],
                };

                // Tratamos tudo o que não for none ou block como inline
                // Para adicionarmos um inline node, precisamos de
                // um inline container
                // Um inline container será o próprio pai se o mesmo
                // for um inline node
                // Ou um anonymous layout box caso o pai seja do tipo bloco
                let inline_container = match styled_node.specified_properties.get("display") {
                    Some(CSSValue::Keyword(ref s)) if s == "block" => {
                        // Obter o último node
                        // Se for o anonymous box, retornamos ele
                        // Caso não for, criamos um
                        match layout_box.children.last_mut() {
                            Some(anonymous_box @ &mut LayoutBox { box_type: BoxType::AnonymousBlock, .. }) => anonymous_box,
                            _ => {
                                layout_box.children.push(anonymous_box);
                                layout_box.children.last_mut().unwrap()
                            }
                        }
                    },
                    _ => &mut layout_box, 
                };

                inline_container.children.push(layout_node(child))
            },
        }
    }

    layout_box
}