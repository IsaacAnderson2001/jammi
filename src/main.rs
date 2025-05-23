use macroquad::prelude::*;

#[macroquad::main("Ratz!")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        next_frame().await;
    }
}
