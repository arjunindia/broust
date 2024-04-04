use ::std::env;

mod lexer;
mod networking;

use macroquad::prelude::*;

const CUR_X: f32 = 13.0;
const CUR_Y: f32 = 18.0;
const SCROLL_DISTANCE: f32 = 100.0;

fn layout(text: &String) -> Vec<(f32, f32, String)> {
    let mut display_list: Vec<(f32, f32, String)> = Vec::new();
    let mut x = 0.0;
    let mut y = 10.0;
    for c in text.chars() {
        display_list.push((x, y, c.to_string()));
        if x >= screen_width() {
            y += CUR_Y;
            x = 0.0;
        }
        x += CUR_X;
    }
    display_list
}

#[macroquad::main("Text")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Not enough arguments! add a `-- {{url}}` at the end of the CLI");
    }
    let font = load_ttf_font("./src/assets/Inter.ttf").await.unwrap();
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
        for (x, y, c) in layout(&text) {
            if y > scroll + screen_height() || y + CUR_Y < scroll {
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
