use crate::board::Board;
use crate::game::GameState;
use macroquad::prelude::*;
use macroquad::ui::{Skin, Style, hash, root_ui};

fn get_screen_window_style() -> (f32, Vec2, Style) {
    let screen_size = screen_width().min(screen_height());
    (
        screen_size,
        vec2(0.5 * screen_size, 0.5 * screen_size),
        root_ui()
            .style_builder()
            .text_color(BLACK)
            .font_size((screen_size / 10.) as u16)
            .build(),
    )
}

pub fn get_menu(game_state: &mut GameState) {
    let (screen_size, window_size, style) = get_screen_window_style();

    if *game_state != GameState::Menu {
        return;
    }
    let ui_skin = Skin {
        window_style: root_ui().style_builder().build(),
        button_style: style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    root_ui().window(
        hash!(),
        vec2(
            screen_size / 2.0 - window_size.x / 2.0,
            screen_size / 2.0 - window_size.y / 2.0,
        ),
        window_size,
        |ui| {
            if ui.button(vec2(screen_size * 0.15, screen_size * 0.1), "Play") {
                *game_state = GameState::Playing;
            }
            if ui.button(vec2(screen_size * 0.15, screen_size * 0.3), "Quit") {
                std::process::exit(0);
            }
        },
    );
}

pub fn game_over(game_state: &mut GameState, board: &mut Board, message: &str) {
    let (screen_size, window_size, style) = get_screen_window_style();

    let ui_skin = Skin {
        window_style: root_ui().style_builder().build(),
        button_style: style.clone(),
        label_style: style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    root_ui().window(
        hash!(),
        vec2(
            screen_size / 2.0 - window_size.x / 2.0,
            screen_size / 2.0 - window_size.y / 2.0,
        ),
        window_size,
        |ui| {
            ui.label(vec2(40.0, 0.0), message);
            if ui.button(vec2(15.0, 75.0), "Play Again?") {
                *game_state = GameState::Playing;
                *board = Board::default();
            }
            if ui.button(vec2(100.0, 155.0), "Quit") {
                std::process::exit(0);
            }
        },
    );
}
