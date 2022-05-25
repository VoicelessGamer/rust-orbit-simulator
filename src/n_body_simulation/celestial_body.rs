use crate::math::vector_3::*;

const GRAVITATIONAL_CONSTANT: f32 = 0.0000000000674;

#[derive(Copy, Clone)]
pub struct CelestialBody {
    id: i32,
    pub mass: f32,
    pub radius: f32,
    pub initial_velocity: Vector3,
    current_velocity: Vector3,
    current_position: Vector3
}

impl CelestialBody {
    // No args constructor
    fn new() -> Self {
        CelestialBody {
            id: -1,
            mass: 1.0, 
            radius: 1.0, 
            initial_velocity: Vector3 {x:0.0, y:0.0, z:0.0}, 
            current_velocity: Vector3 {x:0.0, y:0.0, z:0.0},
            current_position: Vector3 {x:0.0, y:0.0, z:0.0}
        }
    }

    // All args constructor
    fn new_all(id: i32, mass: f32, radius: f32, initial_velocity: Vector3, current_velocity: Vector3, current_position: Vector3) -> Self {
        CelestialBody {
            id: id,
            mass: mass, 
            radius: radius, 
            initial_velocity: initial_velocity, 
            current_velocity: current_velocity,
            current_position: current_position
        }
    }

    /**
     * Updates the velocity of this celestial body based on the values from all given other bodies 
     */
    pub fn update_velocity(&mut self, other_bodies: Vec<CelestialBody>, time_step: f32) {

        // Iterates the other celestial bodies an updates the current velocity 
        // based on the gravitational pull from each
        for body in other_bodies {
            // Check the body is not equal to this body
            // TODO: Need to improve this check
            if body.id != self.id {
                continue;
            }

            // Update the current velocity based of the other body
            let sqr_dist: f32 = body.current_position.sqr_magnitude(self.current_position);
            let dist: f32 = sqr_dist.sqrt();
            let force_dir: Vector3 = (body.current_position - self.current_position) / dist;
            let force: Vector3 = force_dir * GRAVITATIONAL_CONSTANT * self.mass * body.mass / sqr_dist;
            let acceleration: Vector3 = force / self.mass;

            self.current_velocity = acceleration * time_step;
        }
    }

    /**
     * Updates the position of this body based on the current velocity and the time since the last update
     */
    pub fn update_position(&mut self, time_step: f32) {
        self.current_position = self.current_velocity * time_step;
    }
}