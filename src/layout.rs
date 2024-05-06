use crate::dom::TreeNode;
use macroquad::prelude::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct DefaultFont {
    roman: Font,
    italic: Font,
    bold: Font,
    bold_italic: Font,
    mono: Font,
    bold_mono: Font,
}
impl<'a> Default for DefaultFont {
    fn default() -> Self {
        let mut roman =
            load_ttf_font_from_bytes(include_bytes!("assets/fonts/OpenSans/OpenSans-Regular.ttf"))
                .unwrap();
        let mut italic =
            load_ttf_font_from_bytes(include_bytes!("assets/fonts/OpenSans/OpenSans-Italic.ttf"))
                .unwrap();
        let mut bold =
            load_ttf_font_from_bytes(include_bytes!("assets/fonts/OpenSans/OpenSans-Bold.ttf"))
                .unwrap();
        let mut bold_italic = load_ttf_font_from_bytes(include_bytes!(
            "assets/fonts/OpenSans/OpenSans-BoldItalic.ttf"
        ))
        .unwrap();
        let mut mono = load_ttf_font_from_bytes(include_bytes!(
            "assets/fonts/JetbrainsMono/JetBrainsMono-Regular.ttf"
        ))
        .unwrap();
        let mut bold_mono = load_ttf_font_from_bytes(include_bytes!(
            "assets/fonts/JetbrainsMono/JetBrainsMono-Bold.ttf"
        ))
        .unwrap();
        roman.set_filter(FilterMode::Nearest);
        italic.set_filter(FilterMode::Nearest);
        bold.set_filter(FilterMode::Nearest);
        bold_italic.set_filter(FilterMode::Nearest);
        mono.set_filter(FilterMode::Nearest);
        bold_mono.set_filter(FilterMode::Nearest);
        Self {
            roman,
            italic,
            bold,
            bold_italic,
            mono,
            bold_mono,
        }
    }
}

pub struct Layout<'a> {
    pub display_list: Vec<(f32, f32, u16, String, TextDimensions, &'a Font, Color)>,
    x: f32,
    y: f32,
    style: &'a str,
    weight: &'a str,
    font_size: u16,
    r#type: String,
    color: Color,
    print: bool,
}

impl<'a> Layout<'a> {
    pub fn new(r#type: String) -> Self {
        Self {
            display_list: Vec::new(),
            x: 0.0,
            y: 10.0,
            style: "roman",
            weight: "normal",
            font_size: 16,
            r#type,
            color: BLACK,
            print: true,
        }
    }
    fn cached_measure<'b>(
        cache: &mut HashMap<String, TextDimensions>,
        text: &'b str,
        style: &'b str,
        weight: &'b str,
        font: &'b Font,
        font_size: u16,
        font_scale: f32,
    ) -> TextDimensions {
        if !cache.is_empty() {
            let key = format!("{}{}{}{}{}", style, weight, text, font_size, font_scale);
            if cache.contains_key(&key) {
                return cache.get(&key).unwrap().to_owned();
            }
        }
        let key = format!("{}{}{}{}{}", style, weight, text, font_size, font_scale); // text third  cuz space
                                                                                     // characters wont be
                                                                                     // removed
        cache.insert(
            key.clone(),
            measure_text(text, Some(font), font_size, font_scale),
        );
        cache.get(&key.to_string()).unwrap().to_owned()
    }
    fn flush(&mut self) {
        self.y += 18.0 * 1.25;
        self.x = 0.0;
    }
    fn reset(&mut self) {
        self.display_list.clear();
        self.x = 0.0;
        self.y = 1.0;
        self.style = "roman";
        self.weight = "normal";
        self.font_size = 16;
    }
    fn open_tag(&mut self, tag: &str) -> &str {
        if tag == "i" || tag == "em" {
            self.style = "italic";
        } else if tag == "b" || tag == "strong" {
            self.weight = "bold";
        } else if tag == "small" {
            self.font_size -= 2;
        } else if tag == "big" {
            self.font_size += 4;
        } else if tag == "h1" {
            self.flush();
            self.font_size += 16;
            self.flush();
        } else if tag == "br" || tag == "br/" || tag == "hr" || tag == "hr/" || tag == "p" {
            self.flush();
        } else if tag == "code" || tag == "pre" {
            self.style = "mono";
        } else if tag == "a" {
            self.color = BLUE;
        } else if tag == "head" || tag == "style" || tag == "script" {
            self.print = false;
        }
        ""
    }
    fn close_tag(&mut self, tag: &str) {
        if tag == "i" || tag == "em" {
            self.style = "roman";
        } else if tag == "b" || tag == "strong" {
            self.weight = "normal";
        } else if tag == "small" {
            self.font_size += 2;
        } else if tag == "big" {
            self.font_size -= 4;
        } else if tag == "h1" {
            self.font_size -= 16;
        } else if tag == "code" || tag == "pre" {
            self.style = "roman";
        } else if tag == "a" {
            self.color = BLACK;
        } else if tag == "head" || tag == "style" || tag == "script" {
            self.print = true
        }
    }
    fn recurse(
        &mut self,
        font: &'a DefaultFont,
        cache: &mut HashMap<String, TextDimensions>,
        node: &Rc<RefCell<TreeNode>>,
    ) {
        match &node.try_borrow().unwrap().value {
            crate::dom::Element::Text(text) => {
                if !self.print {
                    return;
                }
                let cfont = if self.style == "italic" && self.weight == "bold" {
                    &font.bold_italic
                } else if self.style == "mono" && self.weight == "bold" {
                    &font.bold_mono
                } else if self.weight == "bold" {
                    &font.bold
                } else if self.style == "italic" {
                    &font.italic
                } else if self.style == "mono" {
                    &font.mono
                } else {
                    &font.roman
                };
                let text = html_escape::decode_html_entities(text);

                for word in text.split_whitespace() {
                    self.word(cfont, cache, word, 0, None);
                }
            }
            crate::dom::Element::Tag(tag) => {
                self.open_tag(&tag.tag);
                for child in &node.try_borrow().unwrap().children {
                    self.recurse(font, cache, child);
                }
                self.close_tag(&tag.tag);
            }
        }
    }
    fn recurse_source(
        &mut self,
        font: &'a DefaultFont,
        cache: &mut HashMap<String, TextDimensions>,
        node: &Rc<RefCell<TreeNode>>,
        indent: u32,
    ) {
        match &node.try_borrow().unwrap().value {
            crate::dom::Element::Text(text) => {
                for word in text.split_whitespace() {
                    self.word(&font.roman, cache, word, indent + 1, None);
                }
                self.flush();
            }
            crate::dom::Element::Tag(tag) => {
                self.word(&font.bold, cache, format!("<").as_str(), indent, None);
                self.word(
                    &font.bold,
                    cache,
                    format!("{}", &tag.tag).as_str(),
                    indent,
                    Some(Color::from_hex(0xf55e5e)),
                );
                self.word(&font.bold, cache, format!(">").as_str(), indent, None);
                self.flush();
                for child in &node.try_borrow().unwrap().children {
                    self.recurse_source(font, cache, child, indent + 2);
                }
                self.word(&font.bold, cache, format!("</").as_str(), indent, None);
                self.word(
                    &font.bold,
                    cache,
                    format!("{}", &tag.tag).as_str(),
                    indent,
                    Some(Color::from_hex(0xf55e5e)),
                );
                self.word(&font.bold, cache, format!(">").as_str(), indent, None);
                self.flush();
            }
        }
    }

    fn word(
        &mut self,
        cfont: &'a Font,
        cache: &mut HashMap<String, TextDimensions>,
        word: &str,
        indent: u32,
        color: Option<Color>,
    ) {
        let color = color.unwrap_or(self.color);
        let space_measure = Self::cached_measure(
            cache,
            " ",
            self.style,
            self.weight,
            cfont,
            self.font_size,
            1.0,
        );

        let tab_measure = Self::cached_measure(
            cache,
            " ",
            self.style,
            self.weight,
            cfont,
            self.font_size,
            1.0,
        );
        let measure: TextDimensions = Self::cached_measure(
            cache,
            word,
            self.style,
            self.weight,
            cfont,
            self.font_size,
            1.0,
        );
        if self.x <= 0.0 {
            self.x = 0.0 + (tab_measure.width * indent as f32).clamp(0.0, screen_width());
        }

        if self.x + measure.width >= screen_width() {
            self.y += 18.0 * 1.25;
            self.x = 0.0 + (tab_measure.width * indent as f32);
        }
        self.display_list.push((
            self.x,
            self.y,
            self.font_size,
            word.to_string(),
            measure,
            cfont,
            color,
        ));
        self.x += measure.width + space_measure.width;
    }
    pub fn layout(
        &mut self,
        cache: &mut HashMap<String, TextDimensions>,
        node: &Rc<RefCell<TreeNode>>,
        font: &'a DefaultFont,
    ) {
        self.reset();
        if self.r#type == "source" {
            self.recurse_source(font, cache, node, 0);
        } else {
            self.recurse(font, cache, node);
        }
    }
}
