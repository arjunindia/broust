use crate::lexer::{Tag, Text, Token};
use macroquad::prelude::*;
use std::collections::HashMap;

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
    pub display_list: Vec<(f32, f32, u16, String, TextDimensions, &'a Font)>,
    x: f32,
    y: f32,
    style: &'a str,
    weight: &'a str,
}

impl<'a> Layout<'a> {
    pub fn new() -> Self {
        Self {
            display_list: Vec::new(),
            x: 0.0,
            y: 10.0,
            style: "roman",
            weight: "normal",
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
    }
    pub fn layout(
        &mut self,
        cache: &mut HashMap<String, TextDimensions>,
        tokens: &Vec<Token>,
        font: &'a DefaultFont,
    ) {
        self.reset();
        let mut font_size: u16 = 16;
        for token in tokens {
            let c = match &token {
                Token::Text(Text { text }) => text,
                Token::Tag(Tag { tag }) => {
                    if tag == "i" || tag == "em" {
                        self.style = "italic";
                    } else if tag == "/i" || tag == "/em" {
                        self.style = "roman";
                    } else if tag == "b" || tag == "strong" {
                        self.weight = "bold";
                    } else if tag == "/b" || tag == "/strong" {
                        self.weight = "normal";
                    } else if tag == "small" {
                        font_size -= 2;
                    } else if tag == "/small" {
                        font_size += 2;
                    } else if tag == "big" {
                        font_size += 4;
                    } else if tag == "/big" {
                        font_size -= 4;
                    } else if tag == "br" || tag == "br/" || tag == "/p" {
                        self.flush();
                    } else if tag == "code" || tag == "pre" {
                        self.style = "mono";
                    } else if tag == "/code" || tag == "/pre" {
                        self.style = "roman";
                    } else {
                    }
                    ""
                }
            };
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
            let space_measure =
                Self::cached_measure(cache, " ", self.style, self.weight, cfont, font_size, 1.0);
            let empty_measure =
                Self::cached_measure(cache, "", self.style, self.weight, cfont, font_size, 1.0);
            let c = html_escape::decode_html_entities(c);

            for word in c.split_whitespace() {
                let measure: TextDimensions = Self::cached_measure(
                    cache,
                    word,
                    self.style,
                    self.weight,
                    cfont,
                    font_size,
                    1.0,
                );
                if self.x + measure.width >= screen_width() {
                    self.y += 18.0 * 1.25;
                    self.x = 0.0;
                }
                self.display_list.push((
                    self.x,
                    self.y,
                    font_size,
                    word.to_string(),
                    measure,
                    cfont,
                ));
                self.x += measure.width + space_measure.width;
            }
            if c.split_whitespace().count() <= 0 {
                self.display_list.push((
                    self.x,
                    self.y,
                    font_size,
                    "".to_string(),
                    empty_measure,
                    cfont,
                ));
            }
        }
    }
}
