// “Escape! Code Your Way Out of a Paper Bag”
//

use macroquad::prelude::*;

const WORD_WIDTH: f32 = 800.;
const WORD_HEIGHT: f32 = 600.;
const BAG_WIDTH: f32 = 790.;
const BAG_HEIGHT: f32 = 590.;


struct Bag {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct Turtle {
    pos: Vec2,
    // speed: f32,
    // energy: f32,
}

struct World {
    width: f32,
    height: f32,
    bag: Bag,
    turtle: Turtle,
    path: Vec<Vec2>,
}


impl Bag {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Bag {
        Bag { x, y, width, height }
    }
}


impl Turtle {
    pub fn new(pos_x: f32, pos_y: f32) -> Turtle {
        let pos = Vec2::new(pos_x, pos_y);
        Turtle { pos } 
    }
}


impl World {
    pub fn new(width: f32, height: f32, bag: Bag, turtle: Turtle, path: Vec<Vec2>) -> World {
        World {width, height, bag, turtle, path}
    }
}

fn draw_world(world: &World) {
    // Plot
    clear_background(LIGHTGRAY);
    // Paper bag
    draw_rectangle(world.bag.x, world.bag.y, world.bag.width, world.bag.height, DARKGRAY);
    // Escape path
    for i in 0..world.path.len()-1 {
        draw_line(world.path[i].x, world.path[i].y, world.path[i+1].x, world.path[i+1].y, 2., BLACK);
    }
    // Starting point
    draw_circle(world.turtle.pos.x, world.turtle.pos.y, 5., GREEN);
}


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
    let bag = Bag::new((WORD_WIDTH-BAG_WIDTH) / 2., (WORD_HEIGHT-BAG_HEIGHT) / 2., BAG_WIDTH, BAG_HEIGHT);
    let turtle = Turtle::new(400., 300.);
    let path = vec![turtle.pos, Vec2::new(400., 580.), Vec2::new(780., 580.)];
    let world = World::new(WORD_WIDTH, WORD_HEIGHT, bag, turtle, path);

    loop {
        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        // Draw world
        draw_world(&world);
        
        next_frame().await
    }
}