//use std::time::{SystemTime, UNIX_EPOCH};

use crate::n_body_simulation::celestial_body::*;

const FIXED_TIME_STEP: f32 = 0.02;

pub struct NBodySimulator{
    celestial_bodies: Vec<CelestialBody>
}

#[allow(dead_code)]
impl NBodySimulator {
    /**
     * All args constructor
     */
    pub fn new(celestial_bodies: Vec<CelestialBody>) -> NBodySimulator {
        NBodySimulator {
            celestial_bodies
        }
    }

    /**
     * Function to update the velocity and positions of all celestial bodies
     */
    fn process_physics(&mut self, delta: f32) {
        // Creating a mutable clone of the celestial_bodies vector
        // so that they can be passed as a parameter to the update_velocity function
        // within the mutable for loop iteration below 
        let mut all_bodies = self.celestial_bodies.clone();

        // Iterates the celestial bodies and updates their velocities based on the other bodies
        for body in self.celestial_bodies.iter_mut() {
            // Temporarily excluding the first body, assuming it is the point of reference (i.e. the sun)
            if body.get_id() > 0 {
                body.update_velocity(&mut all_bodies, delta);
            }
        }

        // Iterates the celestial bodies and updates their positions based on the velocities calculated in previous loop
        for body in self.celestial_bodies.iter_mut() {
            // Temporarily excluding the first body, assuming it is the point of reference (i.e. the sun)
            if body.get_id() > 0 {
                body.update_position(delta);
            }
        }
    }

    /**
     * Function to be called by the main entry point to start simulating the orbital physics
     */
    pub fn start(&mut self) {

        // Currently only processing a limited number of physics updates for testing purposes
        for _i in 1..125 {    
            self.process_physics(FIXED_TIME_STEP);
        }
    }
}