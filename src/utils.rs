use crate::cons::TILE_SIZE;
pub fn xy2rowcol(x: f32, y: f32) -> (usize, usize) {
    (
        (y / (TILE_SIZE + 1.)) as usize,
        (x / (TILE_SIZE + 1.)) as usize,
    )
}
