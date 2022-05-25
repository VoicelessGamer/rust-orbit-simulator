use crate::n_body_simulation::celestial_body::*;

pub struct NBodySimulator{
    celestial_bodies: Vec<CelestialBody>
    // CHANGE THIS TO ARRY SLICE I GUESS
}

impl NBodySimulator {

    fn process_physics(mut self, delta: f32) {
        let cloned_bodies = self.celestial_bodies.to_vec();

        for body in self.celestial_bodies.iter_mut() {
            body.update_velocity(cloned_bodies, delta);
        }

        for body in self.celestial_bodies.iter_mut() {
            body.update_position(delta);
        }
    }
}