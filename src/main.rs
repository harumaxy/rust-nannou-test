use nannou::{
    color::{self, IntoLinSrgba},
    draw::background::new,
    prelude::*,
};

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let r1 = Rect::from_xy_wh(pt2(100., 100.), vec2(100., 100.));
    let r2 = Rect::from_wh(vec2(100., 100.));
    draw.background().color(BLACK);
    draw.rect().xy(r1.xy()).wh(r1.wh()).color(RED);
    draw.rect()
        .xy(r2.xy())
        .wh(r2.wh())
        .z_degrees(45.)
        .color(BLUE);

    // Window Rect
    let win = app.window_rect(); // forcus している window
    let r3 = Rect::from_wh(vec2(100., 100.));
    draw.rect()
        .wh(r3.wh())
        .xy(win.wh() / 2.0 - r3.wh() / 2.0)
        .color(GREEN);

    // Window (& Rect) Padding
    let win_p = win.pad(30.);
    draw.rect()
        .wh(r3.pad(20.).wh())
        .xy(win_p.wh() / 2.0 - r3.wh() / 2.0)
        .color(PURPLE);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run()
}
