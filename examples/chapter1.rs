// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;

const WORD_WIDTH: usize = 800;
const WORD_HEIGHT: usize = 600;
const BAG_WIDTH: usize = 790;
const BAG_HEIGHT: usize = 590;


fn window_conf() -> Conf {
    Conf {
        window_title: "Chapter 1".to_owned(),
        fullscreen: false,
        window_width: WORD_WIDTH as i32,
        window_height: WORD_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        clear_background(LIGHTGRAY);
        // Paper bag
        draw_rectangle(
            ((WORD_WIDTH-BAG_WIDTH) / 2) as f32, 
            ((WORD_HEIGHT-BAG_HEIGHT) / 2) as f32, 
            BAG_WIDTH as f32, 
            BAG_HEIGHT as f32, 
            DARKGRAY);
        // Starting point
        draw_circle(100., 100., 10., RED);
        // Escape path
        draw_line(100.0, 100.0, 780., 580., 2., BLACK);

        next_frame().await
    }
}