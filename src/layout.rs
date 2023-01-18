use crate::{
    css::{CSSUnit, CSSValue},
    style::StyledNode,
};

#[derive(Debug, Default, Clone)]
pub struct BoxDimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

impl BoxDimensions {
    pub fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }

    pub fn border_box(self) -> Rect {
        let border = self.border.clone();

        self.padding_box().expanded_by(border)
    }

    pub fn margin_box(self) -> Rect {
        let margin = self.margin.clone();

        self.padding_box().expanded_by(margin)
    }
}

#[derive(Debug, Default, Clone)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    fn expanded_by(self, edges: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edges.left,
            y: self.y - edges.top,
            width: self.width + edges.left + edges.right,
            height: self.height + edges.top + edges.bottom,
        }
    }
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

pub fn layout_node<'a>(styled_node: &'a StyledNode, mut containing_block: BoxDimensions) -> LayoutBox<'a> {
    // O containing block inicial deve ter a altura 0
    containing_block.content.height = 0.0;

    let mut layout_box = create_layout_box(styled_node);

    layout_box.layout(&containing_block);

    layout_box
}

impl LayoutBox<'_> {
    fn get_style_node(&self) -> &StyledNode {
        if let BoxType::Block(node) | BoxType::Inline(node) = self.box_type {
            node
        } else {
            panic!("Sem um styled node para este layout node")
        }
    }

    fn layout(&mut self, containing_block: &BoxDimensions) {
        // No momento apenas o block layout é implementado
        if let BoxType::AnonymousBlock | BoxType::Inline(_) = self.box_type {
            return;
        }

        // Faz uma passada na árvore de cima para baixo para calcular
        // as larguras das caixas pais e de baixo para cima
        // para calcular a altura das caixas filhas
        
        // Calcula a largura do bloco em relação ao seu containing block
        self.calculate_block_width(containing_block);

        // Calcula a posição do bloco em relação ao seu containing block
        self.calculate_block_position(containing_block);

        // Calcula a altura do box a partir de seus filhos
        // Assim vamos subindo na pilha de chamadas
        // e quando descemos da pilha de chamadas, nós subimos
        // na árvore alterando as alturas dos blocos
        for child in &mut self.children {
            // Precisamos fazer o layout do filho para obter o margin box
            child.layout(&self.dimensions);

            self.dimensions.content.height += child.dimensions.clone().margin_box().height;
        }

        // Substitui a altura do bloco pela propriedade `height`
        // Se não houver, irá ser calculado automaticamente.
        if let Some(&CSSValue::Length(height, CSSUnit::Px)) = self.get_style_node().specified_properties.get("height") {
            self.dimensions.content.height = height;
        }
    }

    // Calcula a posição do bloco junto com o tamanho do padding/border/margin
    fn calculate_block_position(&mut self, containing_block: &BoxDimensions) {
        let styled_node = self.get_style_node();
        
        let zero = CSSValue::Length(0.0, CSSUnit::Px);

        let margin_top = styled_node.lookup_property_value(["margin-top", "margin"], &zero).to_px();
        let margin_bottom = styled_node.lookup_property_value(["margin-bottom", "margin"], &zero).to_px();
        let padding_top = styled_node.lookup_property_value(["padding-top", "padding"], &zero).to_px();
        let padding_bottom = styled_node.lookup_property_value(["padding-bottom", "padding"], &zero).to_px();
        let border_top = styled_node.lookup_property_value(["border-top", "border"], &zero).to_px();
        let border_bottom = styled_node.lookup_property_value(["border-bottom", "border"], &zero).to_px();

        let d = &mut self.dimensions;

        let content_x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;
        let content_y = containing_block.content.height + containing_block.content.y + margin_top + border_top + padding_top;

        d.margin.top = margin_top;
        d.margin.bottom = margin_bottom;
        d.padding.top = padding_top;
        d.padding.bottom = padding_bottom;
        d.border.top = border_top;
        d.border.bottom = border_bottom;
        d.padding.top = padding_top;
        d.padding.bottom = padding_bottom;

        d.content.x = content_x;
        d.content.y = content_y;
    }

    // Calcula a largura desta block box relativo
    // às dimensões um containing block (que é outra caixa).
    fn calculate_block_width(&mut self, containing_block: &BoxDimensions) {
        let styled_node = self.get_style_node();

        let zero = CSSValue::Length(0.0, CSSUnit::Px);

        let mut margin_left = styled_node.lookup_property_value(["margin-left", "margin"], &zero).to_owned();
        let mut margin_right = styled_node.lookup_property_value(["margin-right", "margin"], &zero).to_owned();

        let border_left = styled_node.lookup_property_value(["border-left", "border"], &zero).to_owned();
        let border_right = styled_node.lookup_property_value(["border-right", "border"], &zero).to_owned();

        let padding_left = styled_node.lookup_property_value(["padding-left", "padding"], &zero).to_owned();
        let padding_right = styled_node.lookup_property_value(["padding-right", "padding"], &zero).to_owned();

        let auto = CSSValue::Keyword("auto".to_owned());
        let mut width = styled_node.lookup_property_value(["width"], &auto).to_owned();

        let total = [&margin_left, &margin_right, &border_left, &border_right, &padding_left, &padding_right, &width]
            .iter()
            .map(|value| value.to_px())
            .sum::<f32>();

        let underflow = containing_block.content.width - total;

        if underflow < 0.0 && width == auto {
            if margin_left == auto {
                margin_left = zero.clone();
            }

            if margin_right == auto {
                margin_right = zero.clone();
            }
        }

        // Distribuir o espaço disponível de tal forma que
        // margin-* + border-* + padding-* + width = largura do containing block
        match (width == auto, margin_left == auto, margin_right == auto) {
            // Se todos os componentes forem automáticos, o overflow deve ser adicionado à margem direita.
            (false, false, false) => {
                margin_right = CSSValue::Length(margin_right.to_px() + underflow, CSSUnit::Px);
            },

            // Caso contrário, se apenas a margem direita ou apenas a margem esquerda forem automáticos, coloque o underflow neles.
            (false, false, true) => {
                margin_right = CSSValue::Length(underflow, CSSUnit::Px);
            },

            (false, true, false) => {
                margin_left = CSSValue::Length(underflow, CSSUnit::Px);
            },
            
            // Caso contrário, se a largura for automática a largura deve ser 
            // o underflow. Se ocorrer overflow, a largura deve ser zero e a
            // margem direita deve receber o overflow.
            // Margens automáticas serão zeradas.
            (true, _, _) => {
                if margin_left == auto {
                    margin_left = zero.clone();
                }

                if margin_right == auto {
                    margin_right = zero.clone();
                }

                if underflow >= 0.0 {
                    width = CSSValue::Length(underflow, CSSUnit::Px);
                } else {
                    width = zero.clone();
                    margin_right = CSSValue::Length(margin_right.to_px() + underflow, CSSUnit::Px);
                }
            },

            // Caso contrário, então a margem direita e esquerda deve ter o overflow dividido igualmente.
            (false, _, _) => {
                margin_left = CSSValue::Length(underflow / 2.0, CSSUnit::Px);
                margin_right = CSSValue::Length(underflow / 2.0, CSSUnit::Px);
            }
        }

        // Adicionar as dimensões à caixa
        self.dimensions.content.width = width.to_px();
        self.dimensions.margin.left = margin_left.to_px();
        self.dimensions.margin.right = margin_right.to_px();
        self.dimensions.border.left = border_left.to_px();
        self.dimensions.border.right = border_right.to_px();
        self.dimensions.padding.left = padding_left.to_px();
        self.dimensions.padding.right = padding_right.to_px();
    }
}

// Construção da layout tree
fn create_layout_box<'a>(styled_node: &'a StyledNode) -> LayoutBox<'a> {
    let mut layout_box = LayoutBox {
        box_type: match styled_node.specified_properties.get("display") {
            Some(CSSValue::Keyword(ref s)) if s == "block" => BoxType::Block(styled_node),
            Some(CSSValue::Keyword(ref s)) if s == "none" => panic!(
                "Não é possível construir uma layout box para um nó raiz que tem display: none"
            ),
            _ => BoxType::Inline(styled_node),
        },
        dimensions: Default::default(),
        children: vec![],
    };

    for child in &styled_node.children {
        match child.specified_properties.get("display") {
            Some(CSSValue::Keyword(ref s)) if s == "block" => {
                layout_box.children.push(create_layout_box(child))
            },
            Some(CSSValue::Keyword(ref s)) if s == "none" => {},
            _ => {
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
                            Some(
                                anonymous_box @ &mut LayoutBox {
                                    box_type: BoxType::AnonymousBlock,
                                    ..
                                },
                            ) => anonymous_box,
                            _ => {
                                let anonymous_box = LayoutBox {
                                    dimensions: Default::default(),
                                    box_type: BoxType::AnonymousBlock,
                                    children: vec![],
                                };
                                layout_box.children.push(anonymous_box);
                                layout_box.children.last_mut().unwrap()
                            }
                        }
                    }
                    _ => &mut layout_box,
                };

                inline_container.children.push(create_layout_box(child))
            }
        }
    }

    layout_box
}
