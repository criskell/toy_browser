use crate::{
    css::{CSSValue, SimpleSelector, Stylesheet},
    dom::{Element, Node},
};
use std::collections::HashMap;

type PropertyMap = HashMap<String, CSSValue>;

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_properties: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

impl<'a> StyledNode<'a> {
    pub fn lookup_property_value<'b, P>(&'a self, properties: P, default: &'a CSSValue) -> &'a CSSValue
    where
        P: IntoIterator<Item = &'b str>,
    {
        for property in properties.into_iter() {
            if let Some(value) = self.specified_properties.get(property) {
                return value;
            }
        }

        default
    }
}

pub fn style_node<'a>(node: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    match node {
        Node::Text(_) => StyledNode {
            node,
            children: vec![],
            specified_properties: HashMap::new(),
        },
        Node::Element(element) => StyledNode {
            node,
            children: element
                .children
                .iter()
                .map(|node| style_node(node, stylesheet))
                .collect(),
            specified_properties: get_specified_properties(element, stylesheet),
        },
    }
}

fn get_specified_properties(element: &Element, stylesheet: &Stylesheet) -> PropertyMap {
    let mut specified_properties = PropertyMap::new();

    let mut matched_rules = stylesheet
        .rules
        .iter()
        .filter_map(|rule| {
            rule.selectors
                .iter()
                .find(|selector| selector_matches(selector, element))
                .map(|selector| (selector.specificity(), rule))
        })
        .collect::<Vec<_>>();

    matched_rules.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (_, rule) in matched_rules {
        for declaration in rule.declarations.iter() {
            specified_properties.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    specified_properties
}

fn selector_matches(selector: &SimpleSelector, element: &Element) -> bool {
    if let Some(ref tag_name) = selector.tag_name {
        if tag_name != &element.tag_name {
            return false;
        }
    }

    if let Some(ref id) = selector.id {
        if element
            .attributes
            .get("id")
            .map_or(true, |element_id| id != element_id)
        {
            return false;
        }
    }

    let classes = element.classes();
    if selector
        .classes
        .iter()
        .any(|class| !classes.contains(class.as_str()))
    {
        return false;
    }

    return true;
}
