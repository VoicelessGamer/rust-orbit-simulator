use gdnative::prelude::*;

use crate::n_body_simulation::n_body_simulator::*;
use crate::n_body_simulation::celestial_body::*;
use crate::math::vector_3::Vector3 as Vec3;

#[derive(NativeClass)]
#[inherit(Node)]
#[allow(dead_code)]
pub struct NBodySystem {
    #[property(default = 0.02)]
    fixed_time_step: f64,

    simulator: Option<NBodySimulator>
}

impl NBodySystem {
    fn new(_owner: &Node) -> Self {
        NBodySystem {
            fixed_time_step: 0.02,
            simulator: None
        }
    }
}

#[methods]
impl NBodySystem {

    /**
     * Called on scene load. Sets up the simulator with a fixed time step and no available bodies
     */
    #[export]
    fn _ready(&mut self, _owner: &Node) {

        self.simulator = Some(NBodySimulator::new(self.fixed_time_step, Vec::new()));

        godot_print!("N Body System ready finally!");
    }

    /**
     * Allows a godot script to add a new body to the simulator
     */
    #[export]
    fn add_celestial_body(&mut self, _owner: &Node, id: i32, reference_point: bool, mass: f64, radius: f64, current_velocity: Vector3, current_position: Vector3) {
        let sim = self.simulator.as_mut().unwrap();

        sim.add_celestial_body(CelestialBody::new(
            id,
            reference_point,
            mass,
            radius,
            Vec3 {x: current_velocity.x as f64, y: current_velocity.y as f64, z: current_velocity.z as f64},
            Vec3 {x: current_position.x as f64, y: current_position.y as f64, z: current_position.z as f64}
        ));

        godot_print!("Celestial Body added! Total: {}", sim.celestial_bodies.len());
    }

    /**
     * Performs a number of given steps through the simulation
     */
    #[export]
    fn run_simulation_step(&mut self, _owner: &Node, steps: i32) {
        let sim = self.simulator.as_mut().unwrap();

        sim.run_simulation_step(steps);

        //let earth: CelestialBody = sim.celestial_bodies[1];
        //godot_print!("earth: {},{},{}", earth.current_position.x, earth.current_position.y, earth.current_position.z);
    }

    /**
     * Allows a godot script to retrieve the current position of a given celestial body
     */
    #[export]
    fn get_body_position(&mut self, _owner: &Node, index: usize) -> Option<Vector3> {
        let sim = self.simulator.as_mut().unwrap();

        let body = sim.celestial_bodies[index];

        return Option::Some(Vector3 {x: body.current_position.x as f32, y: body.current_position.y as f32, z: body.current_position.z as f32});
    }
}