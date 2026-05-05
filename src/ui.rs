use crate::board::Board;
use crate::game::GameState;
use macroquad::prelude::*;
use macroquad::ui::{Skin, hash, root_ui};

pub fn get_menu(game_state: &mut GameState) {
    if *game_state != GameState::Menu {
        return;
    }
    let window_size = vec2(370.0, 320.0);
    let window_style = root_ui()
        .style_builder()
        //.background(window_background)
        .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();
    let button_style = root_ui()
        .style_builder()
        //        .background(button_background)
        //       .background_clicked(button_clicked_background)
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        //      .font(&font)
        //.unwrap()
        .text_color(BLACK)
        .font_size(64)
        .build();
    let ui_skin = Skin {
        window_style,
        button_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    root_ui().window(
        hash!(),
        vec2(
            screen_width() / 2.0 - window_size.x / 2.0,
            screen_height() / 2.0 - window_size.y / 2.0,
        ),
        window_size,
        |ui| {
            if ui.button(vec2(65.0, 25.0), "Play") {
                *game_state = GameState::Playing;
            }
            if ui.button(vec2(65.0, 125.0), "Quit") {
                std::process::exit(0);
            }
        },
    );
}

pub fn game_over(game_state: &mut GameState, board: &mut Board, message: &str) {
    let window_size = vec2(370.0, 320.0);
    let window_style = root_ui()
        .style_builder()
        //.background(window_background)
        .background_margin(RectOffset::new(0.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();
    let button_style = root_ui()
        .style_builder()
        //        .background(button_background)
        //       .background_clicked(button_clicked_background)
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        //      .font(&font)
        //.unwrap()
        .text_color(BLACK)
        .font_size(64)
        .build();

    let label_style = root_ui()
        .style_builder()
        //        .background(button_background)
        //       .background_clicked(button_clicked_background)
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        //      .font(&font)
        //.unwrap()
        .text_color(BLACK)
        .font_size(20)
        .build();
    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    root_ui().window(
        hash!(),
        vec2(
            screen_width() / 2.0 - window_size.x / 2.0,
            screen_height() / 2.0 - window_size.y / 2.0,
        ),
        window_size,
        |ui| {
            ui.label(vec2(0.0, 5.0), message);
            if ui.button(vec2(15.0, 50.0), "Play Again?") {
                *game_state = GameState::Playing;
                *board = Board::default();
            }
            if ui.button(vec2(65.0, 155.0), "Quit") {
                std::process::exit(0);
            }
        },
    );
}
