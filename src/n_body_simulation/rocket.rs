use crate::math::vector_3::*;
use crate::n_body_simulation::body::*;

#[derive(Copy, Clone)]
pub struct Rocket {
    mass: f64,
    current_velocity: Vector3,
    pub current_position: Vector3
}

#[allow(dead_code)]
impl Rocket {
    /**
     * All args constructor
     */ 
    pub fn new(mass: f64, current_velocity: Vector3, current_position: Vector3) -> Rocket {
        Rocket {
            mass: mass, 
            current_velocity: current_velocity,
            current_position: current_position
        }
    }
}

/**
 * Implementing the Body trait allows the simulator to take this object into account during the simulation functionality
 */
impl Body for Rocket {
    fn mass(&self) -> &f64 {
        &self.mass
    }

    fn velocity(&self) -> &Vector3{
        &self.current_velocity
    }

    fn position(&self) -> &Vector3{
        &self.current_position
    }

    fn add_velocity(&mut self, force: Vector3) {
        self.current_velocity += force;
    }

    /**
     * Updates the position of this body based on the current velocity and the time since the last update
     */
    fn update_position(&mut self, time_step: f64) {
        self.current_position += self.current_velocity * time_step;
    }
}