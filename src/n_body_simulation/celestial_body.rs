use crate::math::vector_3::*;

//const GRAVITATIONAL_CONSTANT: f32 = 0.0000000000674;
const GRAVITATIONAL_CONSTANT: f32 = 0.674;

#[derive(Copy, Clone)]
pub struct CelestialBody {
    id: i32,
    pub mass: f32,
    pub radius: f32,
    current_velocity: Vector3,
    pub current_position: Vector3
}

#[allow(dead_code)]
impl CelestialBody {
    /**
     * All args constructor
     */ 
    pub fn new(id: i32, mass: f32, radius: f32, current_velocity: Vector3, current_position: Vector3) -> CelestialBody {
        CelestialBody {
            id: id,
            mass: mass, 
            radius: radius, 
            current_velocity: current_velocity,
            current_position: current_position
        }
    }

    /**
     * Temporary function used by the simulator 
     */
    pub fn get_id(&self) -> i32 {
        return self.id;
    }

    /**
     * Updates the velocity of this celestial body based on the values from all given other bodies 
     */
    pub fn update_velocity(&mut self, other_bodies: &[CelestialBody], time_step: f32) {

        // Iterates the other celestial bodies an updates the current velocity 
        // based on the gravitational pull from each
        for body in other_bodies {
            // Check the body is not equal to this body
            // TODO: Need to improve this check
            if body.id == self.id {
                continue;
            }

            // Update the current velocity based of the other body
            let sqr_dist: f32 = body.current_position.sqr_magnitude(self.current_position);
            let dist: f32 = sqr_dist.sqrt(); // Magnitude (distance between points)
            let force_dir: Vector3 = (body.current_position - self.current_position) / dist;
            let force: Vector3 = force_dir * GRAVITATIONAL_CONSTANT * body.mass / sqr_dist;

            self.current_velocity += force * time_step;
        }
    }

    /**
     * Updates the position of this body based on the current velocity and the time since the last update
     */
    pub fn update_position(&mut self, time_step: f32) {
        self.current_position += self.current_velocity * time_step;

        //println!("Body: {}, is in position: ({}, {}, {})", self.id, self.current_position.x, self.current_position.y, self.current_position.z);
        println!("{},{}", self.current_position.x, self.current_position.z);
    }
}