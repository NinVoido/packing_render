use nannou::prelude::*;
use packing_lib::annealing::annealing::Anneal;

struct Model {
    state: Anneal,
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    let cords = packing_lib::load_utils::load_file("cords".to_string()).unwrap();
    Model { state: Anneal::new(cords)}
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.state.step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(DARKORCHID);

    for p in &model.state.cords {
        debug_assert_eq!(p.0.len(), 2);
        draw.ellipse().x(p.0[0] as f32 * 10.0).y(p.0[1] as f32 * 10.0).radius(3.0).color(GHOSTWHITE).finish();
    }
    draw.to_frame(app, &frame).unwrap();
}
