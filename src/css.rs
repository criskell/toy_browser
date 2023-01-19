use std::u8;

#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<SimpleSelector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub value: CSSValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CSSValue {
    Keyword(String),
    Length(f32, CSSUnit),
    Color(Color),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CSSUnit {
    Px,
}

pub fn parse(input: String) -> Stylesheet {
    let mut parser = Parser { input, cursor: 0 };

    parser.consume_stylesheet()
}

pub type Specificity = (usize, usize, usize);

impl SimpleSelector {
    pub fn specificity(&self) -> Specificity {
        let a = self.id.iter().count();
        let b = self.classes.len();
        let c = self.tag_name.iter().count();

        (a, b, c)
    }
}

impl CSSValue {
    pub fn to_px(&self) -> f32 {
        match self {
            &CSSValue::Length(length, CSSUnit::Px) => length,
            _ => 0.0
        }
    }
}

struct Parser {
    input: String,
    cursor: usize,
}

impl Parser {
    fn consume_stylesheet(&mut self) -> Stylesheet {
        Stylesheet {
            rules: self.consume_rules(),
        }
    }

    fn consume_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();

        self.consume_whitespace();

        while !self.eof() {
            rules.push(self.consume_rule());
            self.consume_whitespace();
        }

        rules
    }

    fn consume_rule(&mut self) -> Rule {
        Rule {
            selectors: self.consume_selectors(),
            declarations: self.consume_declaration_block(),
        }
    }

    fn consume_selectors(&mut self) -> Vec<SimpleSelector> {
        let mut selectors = Vec::new();

        while !self.eof() && self.peek() != '{' {
            self.consume_whitespace();
            selectors.push(self.consume_simple_selector());

            if self.peek() == ',' {
                self.consume_char();
            }
        }

        selectors.sort_by(|a, b| a.specificity().cmp(&b.specificity()));

        selectors
    }

    fn consume_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new(),
        };

        while !self.eof() && self.peek() != '{' && self.peek() != ',' {
            match self.peek() {
                '.' => {
                    self.consume_char();
                    selector.classes.push(self.consume_word());
                }
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.consume_word());
                }
                '*' => {
                    self.consume_char();
                }
                _ => {
                    selector.tag_name = Some(self.consume_word());
                }
            }

            self.consume_whitespace();
        }

        selector
    }

    fn consume_declaration_block(&mut self) -> Vec<Declaration> {
        self.consume_char(); // '{'

        let mut declarations = Vec::new();

        while !self.eof() && self.peek() != '}' {
            self.consume_whitespace();
            declarations.push(self.consume_declaration());
            self.consume_whitespace();

            if self.peek() == ';' {
                self.consume_char();
            }

            self.consume_whitespace();
        }

        self.consume_char(); // '}'

        declarations
    }

    fn consume_declaration(&mut self) -> Declaration {
        let name = self.consume_word();

        self.consume_whitespace();

        self.consume_char(); // ':'

        self.consume_whitespace();

        let value = self.consume_value();

        self.consume_whitespace();

        Declaration { name, value }
    }

    fn consume_value(&mut self) -> CSSValue {
        match self.peek() {
            '#' => self.consume_hex_color(),
            '0'..='9' => self.consume_length(),
            _ => self.consume_keyword(),
        }
    }

    fn consume_hex_color(&mut self) -> CSSValue {
        self.consume_char(); // '#'
        let r = self.consume_hex_value();
        let g = self.consume_hex_value();
        let b = self.consume_hex_value();
        let a = self.consume_hex_value();

        CSSValue::Color(Color { r, g, b, a })
    }

    fn consume_hex_value(&mut self) -> u8 {
        if self.eof() {
            0
        } else {
            let value = u8::from_str_radix(&self.input[self.cursor..=self.cursor + 1], 16)
                .unwrap_or_default();
            self.advance_by(2);
            value
        }
    }

    fn consume_length(&mut self) -> CSSValue {
        let value = self.consume_number();
        let unit = self.consume_unit();

        CSSValue::Length(value, unit)
    }

    fn consume_number(&mut self) -> f32 {
        self.consume_while(is_real_number_digit)
            .parse()
            .unwrap_or_default()
    }

    fn consume_unit(&mut self) -> CSSUnit {
        match self.consume_word().as_str() {
            "px" => CSSUnit::Px,
            unit => panic!("Unidade desconhecida: {}", unit),
        }
    }

    fn consume_keyword(&mut self) -> CSSValue {
        CSSValue::Keyword(self.consume_word())
    }

    fn eof(&self) -> bool {
        self.cursor >= self.input.len()
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.cursor).unwrap()
    }

    fn advance_by(&mut self, by: usize) {
        self.cursor += by
    }

    fn consume_char(&mut self) -> char {
        let c = self.peek();
        self.advance_by(1);
        c
    }

    fn consume_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> String {
        let mut str = String::new();

        while !self.eof() && predicate(self.peek()) {
            str.push(self.consume_char())
        }

        str
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(|c| c.is_whitespace())
    }

    fn consume_word(&mut self) -> String {
        self.consume_while(is_word_char)
    }
}

fn is_word_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '-' | '0'..='9' => true,
        _ => false,
    }
}

fn is_real_number_digit(c: char) -> bool {
    match c {
        '0'..='9' | '.' => true,
        _ => false,
    }
}
