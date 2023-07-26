use packing_lib::annealing::annealing::Anneal;
use packing_lib::math_utils::Point;

use std::iter::zip;
use kiss3d::event::{Action, Key, WindowEvent};

use kiss3d::nalgebra::{Vector3};
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let cords = packing_lib::load_utils::load_file("cords".to_string()).unwrap();
    let mut state = Anneal::new(cords);

    let mut window = Window::new("packing");

    window.set_background_color(0.2, 0.1803921568627451, 0.2549019607843137);
    window.set_light(Light::StickToCamera);

    let mut spheres = Vec::new();
    let mut add_pnt = window.add_sphere(0.2);
    add_pnt.set_color(1.0, 0.0, 0.0);

    for _ in &state.cords {
        spheres.push(window.add_sphere(0.1));
        spheres.last_mut().unwrap().set_color(0.6941176470588235, 0.8901960784313725, 0.6784313725490196);
    }

    while window.render() {

        for i in window.events().iter() {
            match i.value {
                WindowEvent::Key(Key::Left, Action::Press, _) => {
                    add_pnt.prepend_to_local_translation(&Vector3::new(0.1, 0.0, 0.0).into());
                },
                WindowEvent::Key(Key::Right, Action::Press, _) => {
                    add_pnt.prepend_to_local_translation(&Vector3::new(-0.1, 0.0, 0.0).into());
                },
                WindowEvent::Key(Key::Up, Action::Press, _) => {
                    add_pnt.prepend_to_local_translation(&Vector3::new(0.0, 0.1, 0.0).into());
                },
                WindowEvent::Key(Key::Down, Action::Press, _) => {
                    add_pnt.prepend_to_local_translation(&Vector3::new(0.0, -0.1, 0.0).into());
                },
                WindowEvent::Key(Key::PageUp, Action::Press, _) => {
                    add_pnt.prepend_to_local_translation(&Vector3::new(0.0, 0.0, 0.1).into());
                },
                WindowEvent::Key(Key::PageDown, Action::Press, _) => {
                    add_pnt.prepend_to_local_translation(&Vector3::new(0.0, 0.0, -0.1).into());
                }
                WindowEvent::Key(Key::Space, Action::Press, _) => {
                    spheres.push(window.add_sphere(0.1));
                    spheres.last_mut().unwrap().set_color(0.6941176470588235, 0.8901960784313725, 0.6784313725490196);

                    let mut cords_n = vec![0.0, 0.0, 0.0];
                    let mut trans = add_pnt.data().local_translation();

                    cords_n[0] = trans.x as f64;
                    cords_n[1] = trans.y as f64;
                    cords_n[2] = trans.z as f64;

                    state.cords.push(Point::new(cords_n));
                }
                _ => (),

            }
        }

        window.scene();

        state.step_n(1, 100);

        for (p, sphere) in zip(&state.cords, &mut spheres) {
            sphere.set_local_translation(Vector3::new(p.0[0] as f32, p.0[1] as f32, p.0[2] as f32).into());
        }

    }
}
