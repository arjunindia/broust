use ::std::env;

mod lexer;
mod networking;

use enum_extract::extract;
use lexer::{Text, Token};
use macroquad::prelude::*;

const SCROLL_DISTANCE: f32 = 100.0;

fn layout(tokens: &Vec<Token>, font: &Font) -> Vec<(f32, f32, String, TextDimensions)> {
    let mut display_list: Vec<(f32, f32, String, TextDimensions)> = Vec::new();
    let mut x = 0.0;
    let mut y = 10.0;
    let space_measure = measure_text(" ", Some(font), 16, 1.0);
    println!("sw: {}", screen_width());
    for token in tokens {
        let c = extract!(Token::Text(_), token);
        let t = Text {
            text: "".to_string(),
        };
        let c = &c.unwrap_or(&t).text;
        for word in c.split_whitespace() {
            let measure: TextDimensions = measure_text(word, Some(font), 16, 1.0);
            if x + measure.width >= screen_width() {
                y += measure.height * 1.25;
                x = 0.0;
            }
            display_list.push((x, y, word.to_string(), measure));
            x += measure.width + space_measure.width;
        }
    }
    display_list
}

#[macroquad::main("Text")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Not enough arguments! add a `-- {{url}}` at the end of the CLI");
    }
    let mut font = load_ttf_font("./src/assets/Inter.ttf").await.unwrap();
    font.set_filter(FilterMode::Nearest);
    let text = networking::url::URL::new(&args[1]);
    let text = &text.request();
    let text = lexer::lex(text.to_string());
    let mut scroll = 0.0;
    loop {
        clear_background(WHITE);
        let (_mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
        if mouse_wheel_y < 0.0 {
            scroll += SCROLL_DISTANCE;
        } else if mouse_wheel_y > 0.0 {
            scroll -= SCROLL_DISTANCE;
        }
        for (x, y, c, d) in layout(&text, &font) {
            if (y > scroll + screen_height()) || (y + d.height < scroll) {
                continue;
            }

            draw_text_ex(
                &c,
                x,
                y - scroll,
                TextParams {
                    font: Some(&font),
                    font_size: 16,
                    color: BLACK,
                    ..Default::default()
                },
            );
        }
        next_frame().await
    }
}
