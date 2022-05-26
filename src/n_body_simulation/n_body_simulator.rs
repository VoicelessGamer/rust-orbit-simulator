//use std::time::{SystemTime, UNIX_EPOCH};

use crate::n_body_simulation::celestial_body::*;

pub struct NBodySimulator{
    time_step: f32,
    pub celestial_bodies: Vec<CelestialBody>
}

#[allow(dead_code)]
impl NBodySimulator {
    /**
     * All args constructor
     */
    pub fn new(time_step: f32, celestial_bodies: Vec<CelestialBody>) -> NBodySimulator {
        NBodySimulator {
            time_step,
            celestial_bodies
        }
    }

    /**
     * Add the given celestial body to the stored vector
     */
    pub fn add_celestial_body(&mut self, celestial_body: CelestialBody) {
        self.celestial_bodies.push(celestial_body);
    }

    /**
     * Function to update the velocity and positions of all celestial bodies
     */
    fn process_physics_step(&mut self) {
        // Creating a mutable clone of the celestial_bodies vector
        // so that they can be passed as a parameter to the update_velocity function
        // within the mutable for loop iteration below 
        let mut all_bodies = self.celestial_bodies.clone();

        // Iterates the celestial bodies and updates their velocities based on the other bodies
        for body in self.celestial_bodies.iter_mut() {
            // Temporarily excluding the first body, assuming it is the point of reference (i.e. the sun)
            if body.get_id() > 0 {
                body.update_velocity(&mut all_bodies, self.time_step);
            }
        }

        // Iterates the celestial bodies and updates their positions based on the velocities calculated in previous loop
        for body in self.celestial_bodies.iter_mut() {
            // Temporarily excluding the first body, assuming it is the point of reference (i.e. the sun)
            if body.get_id() > 0 {
                body.update_position(self.time_step);
            }
        }
    }

    /**
     * Function to be called to simulate a given number of physics update calls
     */
    pub fn run_simulation_step(&mut self, steps: i32) {

        // Processes the physics update a desired number of times
        for _i in 0..steps {    
            self.process_physics_step();
        }
    }
}