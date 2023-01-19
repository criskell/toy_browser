use std::iter::repeat;

use crate::{css::{Color, CSSValue}, layout::{Rect, LayoutBox, BoxType}};

#[derive(Debug)]
enum DisplayCommand {
    SolidColor(Color, Rect),
}

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: repeat(Color { r: 0xff, g: 0xff, b: 0xff, a: 0xff })
                .take(width * height)
                .collect()
        }
    }

    fn handle_command(&mut self, command: &DisplayCommand) {
        match command {
            &DisplayCommand::SolidColor(ref color, ref rect) => {
                let x_start = rect.x.clamp(0.0, self.width as f32) as usize;
                let y_start = rect.y.clamp(0.0, self.height as f32) as usize;

                let x_end = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y_end = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in y_start .. y_end {
                    for x in x_start .. x_end {
                        self.pixels[y * self.width + x] = color.clone();
                    }
                }
            }
        }
    }
}

pub fn paint_node(node: &LayoutBox, bounds: Rect) -> Canvas {
    let mut display_list = vec![];

    render_node(&mut display_list, node);

    let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);

    for command in display_list {
        canvas.handle_command(&command);
    }

    canvas
}

fn render_node(display_list: &mut Vec<DisplayCommand>, node: &LayoutBox) {
    // Desenha o fundo (a border-box, mas sem cor agora)
    if let Some(color) = get_color(node, "background-color") {
        display_list.push(DisplayCommand::SolidColor(color, node.dimensions.clone().border_box()));
    }

    // Desenha as bordas se uma cor for especificada
    if let Some(color) = get_color(node, "border-color") {
        let d = node.dimensions.clone();
        let border_box = d.clone().border_box();

        // Borda superior
        display_list.push(DisplayCommand::SolidColor(color.clone(), Rect {
            x: border_box.x,
            y: border_box.y,
            width: border_box.width,
            height: d.border.top,
        }));

        // Borda esquerda
        display_list.push(DisplayCommand::SolidColor(color.clone(), Rect {
            x: border_box.x,
            y: border_box.y,
            width: d.border.left,
            height: border_box.height,
        }));

        // Borda direita
        display_list.push(DisplayCommand::SolidColor(color.clone(), Rect {
            x: border_box.x + border_box.width - d.border.right,
            y: border_box.y,
            width: d.border.right,
            height: border_box.height,
        }));

        // Borda inferior
        display_list.push(DisplayCommand::SolidColor(color.clone(), Rect {
            x: border_box.x,
            y: border_box.y + border_box.height - d.border.bottom,
            width: border_box.width,
            height: d.border.bottom,
        }));
    }

    for child in &node.children {
        render_node(display_list, child);
    }
}

fn get_color(layout_box: &LayoutBox, property: &str) -> Option<Color> {
    match layout_box.box_type {
        BoxType::Block(styled_node) | BoxType::Inline(styled_node) => match styled_node.specified_properties.get(property) {
            Some(CSSValue::Color(color)) => Some(color.clone()),
            _ => None,
        }

        // Caixas de bloco anônimas não tem cor
        BoxType::AnonymousBlock => None,
    }
}