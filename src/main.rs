use ::std::env;
use std::collections::HashMap;

mod dom;
mod layout;
mod lexer;
mod networking;
use dom::HTMLParser;
use macroquad::prelude::*;
const SCROLL_DISTANCE: f32 = 100.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Broust".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Not enough arguments! add a `-- {{url}}` at the end of the CLI");
    }
    let text = networking::url::URL::new(&args[1]);
    let text = &text.request();
    let tree = HTMLParser::new(text.to_string()).parse();
    println!("{:?}", tree);
    let text = lexer::lex(text.to_string());
    let font = layout::DefaultFont::default();
    let mut curr_w = screen_width();

    let mut cache: HashMap<String, TextDimensions> = HashMap::new();
    let mut layout = layout::Layout::new(&mut cache, &text, &font);
    let mut scroll = 0.0;

    loop {
        clear_background(WHITE);
        let (_mouse_wheel_x, mouse_wheel_y) = mouse_wheel();

        if curr_w != screen_width() {
            layout = layout::Layout::new(&mut cache, &text, &font);
            curr_w = screen_width();
        }

        if mouse_wheel_y < 0.0 {
            scroll += SCROLL_DISTANCE;
        } else if mouse_wheel_y > 0.0 {
            scroll -= SCROLL_DISTANCE;
        }
        for (x, y, font_size, c, d, style) in &layout.display_list {
            if (*y > scroll + screen_height()) || (y + d.height < scroll) {
                continue;
            }

            draw_text_ex(
                &c,
                *x,
                y - scroll,
                TextParams {
                    font: Some(&style),
                    font_size: *font_size,
                    color: BLACK,
                    ..Default::default()
                },
            );
        }
        next_frame().await
    }
}
