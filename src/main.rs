use ::std::env;
use core::f64;
use notan::prelude::*;
use notan::text::*;

mod networking;

fn lex(body: String) -> String {
    let mut in_tag = false;
    let mut text = "".to_string();
    for c in body.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            text.push(c);
        }
    }
    text
}

#[derive(AppState)]
struct State {
    font: Font,
    body: String,
    scroll_x: f32,
    scroll_y: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(TextConfig)
        .draw(draw)
        .update(update)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Not enough arguments! add a `-- {{url}}` at the end of the CLI");
    }
    let url = networking::url::URL::new(&args[1]);
    let response = url.request();
    let body = lex(response);
    let font = gfx
        .create_font(include_bytes!("./assets/Inter.ttf"))
        .unwrap();
    let scroll_x = 0.0;
    let scroll_y = 0.0;
    State {
        font,
        body,
        scroll_x,
        scroll_y,
    }
}

fn update(app: &mut App, state: &mut State) {
    if app.mouse.is_scrolling() {
        let delta_x = app.mouse.wheel_delta.x;
        let delta_y = app.mouse.wheel_delta.y;

        state.scroll_x = (state.scroll_x + delta_x);
        state.scroll_y = (state.scroll_y + delta_y);
    }
}
fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut text = gfx.create_text();
    text.clear_color(Color::WHITE);

    text.add(&state.body)
        .font(&state.font)
        .position(state.scroll_x, state.scroll_y)
        .color(Color::BLACK)
        .size(16.0);

    text.chain("Notan! ").size(50.0).color(Color::RED);

    text.chain("(Using TextExtension)")
        .font(&state.font)
        .size(20.0)
        .color(Color::GRAY.with_alpha(0.5));

    gfx.render(&text);
}
