use nannou::{
    color::{self, IntoLinSrgba},
    draw::background::new,
    prelude::*,
};

/// ```
/// let a = 100;
/// ```

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let points = (1..=100)
        .map(|i| i as f32)
        .map(|i| vec2(i * 5., (i).sin() * 10.));
    draw.polyline().points(points.clone()).color(RED);
    draw.polygon().points(points.clone()).color(RED);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run()
}
