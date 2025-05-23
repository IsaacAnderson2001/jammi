use macroquad::{miniquad::window::screen_size, prelude::*};
struct Line {
    start: Vec2,
    end: Vec2,
}
impl Line {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        return Line { start, end };
    }
    pub fn draw(&self) {
        draw_line(self.start.x, self.start.y, self.end.x, self.end.y, 1., RED);
    }
}

fn get_grid_line(i: usize, top_left: Vec2, bottom_right: Vec2, delta: Vec2) -> (Line, Line) {
    let horizontal = Line::new(
        vec2(top_left.x, top_left.y + delta.y * i as f32),
        vec2(bottom_right.x, top_left.y + delta.y * i as f32),
    );

    let vertical = Line::new(
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
        horizontal.draw();
        vertical.draw();

        i = i + 1;
    }

    draw_circle(top_left.x, top_left.y, 10., GREEN);
    draw_circle(bottom_right.x, bottom_right.y, 10., GREEN);

    // draw_line(
    //     top_left.x,
    //     top_left.y,
    //     bottom_right.x,
    //     bottom_right.y,
    //     1.,
    //     RED,
    // );

    // draw_line(0., 0., screen_width(), screen_height(), 1., RED);
}

#[macroquad::main("Ratz!")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        my_draw_grid();

        next_frame().await;
    }
}
