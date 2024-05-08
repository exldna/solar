use crate::render::{BodyRenderVertex, TrackRenderVertex};
use glam::{DVec2, Mat4, Vec2, Vec3};

#[derive(Copy, Clone)]
pub struct Body {
    position: DVec2,
    velocity: DVec2,
    mass: f64,
}

impl Body {
    pub fn new(position: DVec2, velocity: DVec2, mass: f64) -> Self {
        Self {
            position,
            velocity,
            mass,
        }
    }
}

pub struct Space {
    bodies: Vec<Body>,
    forces: Vec<DVec2>,
}

impl<'a> Space {
    pub fn new(bodies: &[Body]) -> Self {
        Self {
            bodies: Vec::from(bodies),
            forces: vec![DVec2::ZERO; bodies.len()],
        }
    }

    pub fn update(&mut self) {
        self.update_forces();
        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.velocity += self.forces[i] / body.mass;
            body.position += body.velocity;
        }
    }

    fn update_forces(&mut self) {
        self.forces.fill(DVec2::ZERO);
        for i in 0..self.bodies.len() {
            for j in i + 1..self.bodies.len() {
                let force = gravity_force(&self.bodies[i], &self.bodies[j]);
                self.forces[i] += force;
                self.forces[j] -= force;
            }
        }
    }
}

fn gravity_force(body_1: &Body, body_2: &Body) -> DVec2 {
    let force_direction = (body_2.position - body_1.position).normalize();
    let r_squared = body_1.position.distance_squared(body_2.position);
    let force_projection = body_1.mass * body_2.mass / r_squared;
    force_direction * force_projection
}

pub struct SpaceView {
    instances: Vec<BodyRenderVertex>,
    tracks: Vec<Vec<TrackRenderVertex>>,
    view: Mat4,
    update_cnt: u64,
}

impl SpaceView {
    pub fn new(colors: &[Vec3]) -> Self {
        let mut instances = vec![
            BodyRenderVertex {
                body_pos: Vec2::ZERO.to_array(),
                color: [0., 0., 0., 1.]
            };
            colors.len()
        ];
        for (i, instance) in instances.iter_mut().enumerate() {
            instance.color = colors[i].extend(1.).to_array();
        }
        Self {
            instances,
            tracks: vec![Vec::new(); colors.len()],
            view: Mat4::IDENTITY,
            update_cnt: 0,
        }
    }

    pub fn update(&mut self, space: &Space) {
        assert_eq!(self.instances.len(), space.bodies.len());
        for (instance, body) in self.instances.iter_mut().zip(space.bodies.iter()) {
            instance.body_pos = body.position.as_vec2().to_array();
        }
        assert_eq!(self.tracks.len(), space.bodies.len());
        for (track, body) in self.tracks.iter_mut().zip(space.bodies.iter()) {
            track.push(TrackRenderVertex { pos: body.position.as_vec2().to_array() });
        }
        let mut center_mass = DVec2::ZERO;
        let mut total_mass = 0f64;
        for body in space.bodies.iter() {
            center_mass += body.position * body.mass;
            total_mass += body.mass;
        }
        let center_mass = center_mass / total_mass;
        self.view = Mat4::from_translation(-center_mass.extend(0.).as_vec3());
        self.update_cnt = self.update_cnt.overflowing_add(1).0;
    }

    pub fn instances(&self) -> &[BodyRenderVertex] {
        self.instances.as_slice()
    }

    pub fn tracks(&self) -> &[Vec<TrackRenderVertex>] {
        self.tracks.as_slice()
    }

    pub fn view_matrix(&self) -> &Mat4 {
        &self.view
    }
}
