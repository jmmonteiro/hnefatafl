use crate::cons::{NUM_TILES, TILE_MARGIN_SCALE_FACTOR};
use macroquad::prelude::*;

pub fn xy2rowcol(x: f32, y: f32) -> (usize, usize) {
    let tile_size = screen_width().min(screen_height()) / (NUM_TILES as f32);
    (
        (y / (tile_size / TILE_MARGIN_SCALE_FACTOR + 1.)) as usize,
        (x / (tile_size / TILE_MARGIN_SCALE_FACTOR + 1.)) as usize,
    )
}
