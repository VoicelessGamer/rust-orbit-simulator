use crate::math::vector_3::*;

//const GRAVITATIONAL_CONSTANT: f64 = 0.000_000_000_0674;
const GRAVITATIONAL_CONSTANT: f64 = 0.000_000_000_000_674;

#[allow(dead_code)] // Here until radius is used
#[derive(Copy, Clone)]
pub struct CelestialBody {
    id: i32,
    reference_point: bool,
    mass: f64,
    radius: f64,
    current_velocity: Vector3,
    pub current_position: Vector3
}

#[allow(dead_code)]
impl CelestialBody {
    /**
     * All args constructor
     */ 
    pub fn new(id: i32, reference_point: bool, mass: f64, radius: f64, current_velocity: Vector3, current_position: Vector3) -> CelestialBody {
        CelestialBody {
            id: id,
            reference_point: reference_point,
            mass: mass, 
            radius: radius, 
            current_velocity: current_velocity,
            current_position: current_position
        }
    }

    /**
     * Getter for the reference point variable
     */
    pub fn is_reference_point(self) -> bool {
        self.reference_point
    }

    /**
     * Updates the velocity of this celestial body based on the values from all given other bodies 
     */
    pub fn update_velocity(&mut self, other_bodies: &[CelestialBody], time_step: f64) {

        // Iterates the other celestial bodies an updates the current velocity 
        // based on the gravitational pull from each
        for body in other_bodies {
            // Check the body is not equal to this body
            if body == self {
                continue;
            }

            // Update the current velocity based of the other body
            let sqr_dist: f64 = body.current_position.sqr_magnitude(self.current_position);
            let dist: f64 = sqr_dist.sqrt(); // Magnitude (distance between points)
            let force_dir: Vector3 = (body.current_position - self.current_position) / dist;
            let force: Vector3 = force_dir * GRAVITATIONAL_CONSTANT * body.mass / sqr_dist;

            self.current_velocity += force * time_step;
        }
    }

    /**
     * Updates the position of this body based on the current velocity and the time since the last update
     */
    pub fn update_position(&mut self, time_step: f64) {
        self.current_position += self.current_velocity * time_step;

        //println!("Body: {}, is in position: ({}, {}, {})", self.id, self.current_position.x, self.current_position.y, self.current_position.z);
        //println!("{},{}", self.current_position.x, self.current_position.z);
    }
}

/**
 * Overload for equals comparison between Celestial Bodies
 * This assumes all Celestial Bodies have a unique id
 */
impl std::cmp::PartialEq for CelestialBody {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CelestialBody {}