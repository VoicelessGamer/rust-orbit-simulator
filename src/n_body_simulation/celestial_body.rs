use crate::math::vector_3::*;
use crate::n_body_simulation::body::*;

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
}

/**
 * Implementing the Body trait allows the simulator to take this object into account during the simulation functionality
 */
impl Body for CelestialBody {
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