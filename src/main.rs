use glam::{DVec2, Vec3};
use miniquad::EventHandler;

mod render;
mod shader;
mod space;

struct Stage {
    render: render::Render,
    space: space::Space,
    space_view: space::SpaceView,
}

impl Stage {
    fn new() -> Self {
        let bodies = &[
            space::Body::new(DVec2::new(100., 50.), DVec2::X / 5. - DVec2::Y / 5., 150.),
            space::Body::new(DVec2::new(-50., -50.), DVec2::Y / 5., 15.),
            space::Body::new(DVec2::new(50., -50.), DVec2::Y / 5., 5.),
        ];
        Self {
            render: render::Render::new(),
            space: space::Space::new(bodies),
            space_view: space::SpaceView::new(&[Vec3::X, Vec3::Y, Vec3::Z]),
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        self.space.update();
    }

    fn draw(&mut self) {
        self.space_view.update(&self.space);
        self.render.draw(&self.space_view);
    }
}

fn main() {
    let conf = miniquad::conf::Conf::default();
    miniquad::start(conf, move || Box::new(Stage::new()));
}
