use crate::board::Board;
use crate::game::GameState;
use macroquad::prelude::*;
use macroquad::ui::widgets;
use macroquad::ui::{Skin, hash, root_ui};

pub fn get_menu(game_state: &mut GameState) {
    if *game_state != GameState::Menu {
        return;
    }

    let screen_size = screen_width().min(screen_height());
    let window_size = vec2(0.5 * screen_size, 0.5 * screen_size);
    let window_pos = vec2(
        screen_width() / 2.0 - window_size.x / 2.0,
        screen_height() / 2.0 - window_size.y / 2.0,
    );

    let window_style = root_ui().style_builder().build();
    let button_style = root_ui()
        .style_builder()
        .text_color(BLACK)
        .font_size((screen_size / 10.) as u16)
        .build();
    let ui_skin = Skin {
        window_style,
        button_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    // Keying the id on the size forces a fresh window (and position) after a resize.
    widgets::Window::new(hash!(screen_size as u32), window_pos, window_size)
        .movable(false)
        .titlebar(false)
        .ui(&mut root_ui(), |ui| {
            if ui.button(vec2(screen_size * 0.15, screen_size * 0.1), "Play") {
                *game_state = GameState::Playing;
            }
            if ui.button(vec2(screen_size * 0.15, screen_size * 0.3), "Quit") {
                std::process::exit(0);
            }
        });

    root_ui().pop_skin();
}

pub fn game_over(game_state: &mut GameState, board: &mut Board, message: &str) {
    let screen_size = screen_width().min(screen_height());
    let window_size = vec2(0.5 * screen_size, 0.5 * screen_size);
    let window_pos = vec2(
        screen_width() / 2.0 - window_size.x / 2.0,
        screen_height() / 2.0 - window_size.y / 2.0,
    );

    let window_style = root_ui().style_builder().build();
    let button_style = root_ui()
        .style_builder()
        .text_color(BLACK)
        .font_size((screen_size / 15.) as u16)
        .build();

    let label_style = root_ui()
        .style_builder()
        .text_color(BLACK)
        .font_size((screen_size / 20.) as u16)
        .build();
    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    // Keying the id on the size forces a fresh window (and position) after a resize.
    widgets::Window::new(hash!(screen_size as u32), window_pos, window_size)
        .movable(false)
        .titlebar(false)
        .ui(&mut root_ui(), |ui| {
            ui.label(vec2(screen_size * 0.1, screen_size * 0.1), message);
            if ui.button(vec2(screen_size * 0.1, screen_size * 0.2), "Play Again?") {
                *game_state = GameState::Playing;
                *board = Board::default();
            }
            if ui.button(vec2(screen_size * 0.1, screen_size * 0.3), "Quit") {
                std::process::exit(0);
            }
        });

    root_ui().pop_skin();
}
