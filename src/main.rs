use ::std::env;

mod lexer;
mod networking;

use itertools::{self, Itertools};
use lexer::{Tag, Text, Token};
use macroquad::prelude::*;
const SCROLL_DISTANCE: f32 = 100.0;

struct DefaultFont {
    roman: Font,
    italic: Font,
    bold: Font,
    bold_italic: Font,
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
        roman.set_filter(FilterMode::Nearest);
        italic.set_filter(FilterMode::Nearest);
        bold.set_filter(FilterMode::Nearest);
        bold_italic.set_filter(FilterMode::Nearest);
        Self {
            roman,
            italic,
            bold,
            bold_italic,
        }
    }
}

struct Layout<'a> {
    display_list: Vec<(f32, f32, u16, String, TextDimensions, &'a Font)>,
    x: f32,
    y: f32,
    style: &'a str,
    weight: &'a str,
}

impl<'a> Layout<'a> {
    fn new(tokens: &Vec<Token>, font: &'a DefaultFont) -> Self {
        let mut display_list: Vec<(f32, f32, u16, String, TextDimensions, &'a Font)> = Vec::new();
        let mut x = 0.0;
        let mut y = 10.0;
        let mut font_size: u16 = 16;
        let mut style = "roman";
        let mut weight = "normal";
        for token in tokens {
            let c = match &token {
                Token::Text(Text { text }) => text,
                Token::Tag(Tag { tag }) => {
                    if tag == "i" || tag == "em" {
                        style = "italic";
                    } else if tag == "/i" || tag == "/em" {
                        style = "roman";
                    } else if tag == "b" || tag == "strong" {
                        weight = "bold";
                    } else if tag == "/b" || tag == "/strong" {
                        weight = "normal";
                    } else if tag == "small" {
                        font_size -= 2;
                    } else if tag == "/small" {
                        font_size += 2;
                    } else if tag == "big" {
                        font_size += 4;
                    } else if tag == "/big" {
                        font_size -= 4;
                    } else {
                    }
                    ""
                }
            };
            let cfont = if style == "italic" && weight == "bold" {
                &font.bold_italic
            } else if weight == "bold" {
                &font.bold
            } else if style == "italic" {
                &font.italic
            } else {
                &font.roman
            };
            let space_measure = measure_text(" ", Some(cfont), font_size, 1.0);
            let empty_measure = measure_text("", Some(cfont), font_size, 1.0);
            let c = html_escape::decode_html_entities(c);

            for word in c.split_whitespace() {
                let measure: TextDimensions = measure_text(word, Some(cfont), font_size, 1.0);
                if x + measure.width >= screen_width() {
                    y += 18.0 * 1.25;
                    x = 0.0;
                }
                display_list.push((x, y, font_size, word.to_string(), measure, cfont));
                x += measure.width + space_measure.width;
            }
            if c.split_whitespace().count() <= 0 {
                display_list.push((x, y, font_size, "".to_string(), empty_measure, cfont));
            }
        }
        Self {
            display_list,
            x,
            y,
            style,
            weight,
        }
    }
}

#[macroquad::main("Text")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Not enough arguments! add a `-- {{url}}` at the end of the CLI");
    }
    let text = networking::url::URL::new(&args[1]);
    let text = &text.request();
    let text = lexer::lex(text.to_string());
    let font = DefaultFont::default();
    let mut scroll = 0.0;
    loop {
        clear_background(WHITE);
        let (_mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
        if mouse_wheel_y < 0.0 {
            scroll += SCROLL_DISTANCE;
        } else if mouse_wheel_y > 0.0 {
            scroll -= SCROLL_DISTANCE;
        }
        let layout = Layout::new(&text, &font);
        for (x, y, font_size, c, d, style) in layout.display_list {
            if (y > scroll + screen_height()) || (y + d.height < scroll) {
                continue;
            }

            draw_text_ex(
                &c,
                x,
                y - scroll,
                TextParams {
                    font: Some(&style),
                    font_size,
                    color: BLACK,
                    ..Default::default()
                },
            );
        }
        next_frame().await
    }
}
