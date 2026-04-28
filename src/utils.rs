use crate::cons::TILE_SIZE;
// TODO: The math is not 100% correct, does not account for the border between each tile
// but in practice the difference should be small. Fix later if I can be arsed
pub fn xy2rowcol(x: f32, y: f32) -> (usize, usize) {
    ((y / TILE_SIZE) as usize, (x / TILE_SIZE) as usize)
}
