use macroquad::{
    color::GRAY,
    input::is_key_down,
    window::{clear_background, next_frame},
};

#[macroquad::main("Super Rider")]
async fn main() {
    // stage 1
    loop {
        // use macroquad to open a window
        clear_background(GRAY);
        if is_key_down(macroquad::input::KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
    // draw background in window
    // draw car
    // make car shoot bullets
    // make background move downwards
    // make enemy cars that move downwards
    // make enemy cars take damage from bullets
    // give car controls to move left and right
}
