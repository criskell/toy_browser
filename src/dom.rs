pub struct Node {
    pub data: NodeData,
    pub children: Vec<Node>
}

pub enum NodeData {
    Text(String),
    Element {
        tag_name: String,
        attributes: AttrMap
    }
}

pub type AttrMap = HashMap<String, String>;