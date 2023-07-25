use nannou::event::WindowEvent;
use nannou::prelude::*;
use nannou::rand;
use nannou::rand::seq::SliceRandom;

use packing_lib::annealing::annealing::Anneal;
use packing_lib::math_utils::Point;

struct Model {
    state: Anneal,
    cur_n: usize,
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    let cords = packing_lib::load_utils::load_file("cords".to_string()).unwrap();
    Model {
        state: Anneal::new(cords),
        cur_n: 1,
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id, simple } => {
            if let Some(simple) = simple {
                match simple {
                    WindowEvent::MousePressed(_) => {
                        model.state.cords.push(Point::new(vec![
                            (app.mouse.x / 10.0) as f64,
                            (app.mouse.y / 10.0) as f64,
                        ]));
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.state.should_inc() {
        model.cur_n += 1;
    }

    //model.state.step_n(model.cur_n, 100);
    model.state.step_n(1, 100);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    for p in &model.state.cords {
        debug_assert_eq!(p.0.len(), 2);
        draw.ellipse()
            .x(p.0[0] as f32 * 10.0)
            .y(p.0[1] as f32 * 10.0)
            .radius(3.0)
            .color(BLACK)
            .finish();
    }
    draw.to_frame(app, &frame).unwrap();
}
