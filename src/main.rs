use macroquad::{miniquad::window::screen_size, prelude::*};

fn get_grid_line(
    i: usize,
    top_left: Vec2,
    bottom_right: Vec2,
    delta: Vec2,
) -> ((Vec2, Vec2), (Vec2, Vec2)) {
    let horizontal = (
        vec2(top_left.x, top_left.y + delta.y * i as f32),
        vec2(bottom_right.x, top_left.y + delta.y * i as f32),
    );

    let vertical = (
        vec2(top_left.x + delta.x * i as f32, top_left.y),
        vec2(top_left.x + delta.x * i as f32, bottom_right.y),
    );

    return (horizontal, vertical);
}

fn my_draw_grid() {
    const GRID_LINE_COUNT: usize = 10;
    const GRID_DIMENSIONS: Vec2 = Vec2::new(500., 500.);
    const GRID_DIMENSIONS_HALF: Vec2 = vec2(GRID_DIMENSIONS.x / 2., GRID_DIMENSIONS.y / 2.);
    const DELTA: Vec2 = vec2(
        GRID_DIMENSIONS.x / GRID_LINE_COUNT as f32,
        GRID_DIMENSIONS.y / GRID_LINE_COUNT as f32,
    );
    const THICKNESS: f32 = 1.;
    const COLOR: Color = RED;

    let screen_size = Vec2::from(screen_size());
    let screen_center = screen_size / 2.;
    let top_left = screen_center - GRID_DIMENSIONS_HALF;
    let bottom_right = screen_center + GRID_DIMENSIONS_HALF;

    let mut i = 0;
    loop {
        if i > GRID_LINE_COUNT {
            break;
        }

        let (horizontal, vertical) = get_grid_line(i, top_left, bottom_right, DELTA);
        draw_line(
            horizontal.0.x,
            horizontal.0.y,
            horizontal.1.x,
            horizontal.1.y,
            THICKNESS,
            COLOR,
        );
        draw_line(
            vertical.0.x,
            vertical.0.y,
            vertical.1.x,
            vertical.1.y,
            THICKNESS,
            COLOR,
        );

        i = i + 1;
    }

    draw_circle(top_left.x, top_left.y, 10., GREEN);
    draw_circle(bottom_right.x, bottom_right.y, 10., GREEN);
}

#[macroquad::main("Ratz!")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        my_draw_grid();

        next_frame().await;
    }
}
