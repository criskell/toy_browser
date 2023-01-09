use crate::dom::{AttrMap, Node};

struct HTMLParser {
    input: String,
    cursor: usize,
}

impl HTMLParser {
    fn peek(&self) -> char {
        self.input.chars().nth(self.cursor).unwrap()
    }

    fn eof(&self) -> bool {
        self.cursor >= self.input.len()
    }

    fn advance_by(&mut self, by: usize) {
        self.cursor += by
    }

    fn consume_char(&mut self) -> char {
        let c = self.peek();
        self.advance_by(1);
        c
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.cursor..].starts_with(s)
    }

    fn consume_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> String {
        let mut str = String::new();

        while !self.eof() && predicate(self.peek()) {
            str.push(self.consume_char())
        }

        str
    }

    fn consume_node(&mut self) -> Node {
        if self.peek() == '<' {
            self.consume_element()
        } else {
            self.consume_text()
        }
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(|c| c.is_whitespace())
    }

    fn consume_element(&mut self) -> Node {
        self.consume_char(); // '<'

        let tag_name = self.consume_identifier();
        self.consume_whitespace();

        let attributes = self.consume_attributes();
        self.consume_char(); // '>'

        let mut children: Vec<_> = Vec::new();

        let end_tag = format!("</{}>", tag_name);

        while !self.eof() && !self.starts_with(&end_tag) {
            let node = self.consume_node();

            children.push(node);
        }

        self.advance_by(end_tag.len());

        Node::Element {
            tag_name,
            attributes,
            children,
        }
    }

    fn consume_attributes(&mut self) -> AttrMap {
        let mut attributes = AttrMap::new();

        while !self.eof() && self.peek() != '>' {
            let (name, value) = self.consume_attribute();

            attributes.insert(name, value);

            self.consume_whitespace();
        }

        attributes
    }

    fn consume_attribute(&mut self) -> (String, String) {
        let name = self.consume_identifier();

        self.consume_char(); // '='

        let quote = self.consume_char();

        let value = self.consume_while(|c| c != quote);

        self.consume_char(); // quote

        (name, value)
    }

    fn consume_identifier(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '-' | '0'..='9' => true,
            _ => false,
        })
    }

    fn consume_text(&mut self) -> Node {
        Node::Text {
            value: self.consume_while(|c| c != '<'),
        }
    }
}

pub fn parse(input: String) -> Node {
    let mut parser = HTMLParser { cursor: 0, input };

    parser.consume_node()
}
