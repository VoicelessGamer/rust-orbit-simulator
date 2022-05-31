mod n_body_simulation;
mod math;

use n_body_simulation::n_body_simulator::*;
use n_body_simulation::celestial_body:: *;
use math::vector_3::Vector3;

const FIXED_TIME_STEP: f64 = 0.02;

//const GRAVITATIONAL_CONSTANT: f64 = 0.000_000_000_0674;
const GRAVITATIONAL_CONSTANT: f64 = 0.000_000_000_000_674;

fn main() {
    let sun = CelestialBody::new(
        0,
        true,
        100_000.0,
        6.5,
        Vector3 {x: 0.0, y: 0.0, z: 0.0},
        Vector3 {x: 0.0, y: 0.0, z: 0.0}
    );

    let earth = CelestialBody::new(
        1,
        false,
        1.0,
        1.8,
        Vector3 {x: 0.0, y: 0.0, z: 72.4},
        Vector3 {x: 15.0, y: 0.0, z: 0.0}
    );

    let mut celestial_bodies = Vec::new();
    celestial_bodies.push(sun);
    celestial_bodies.push(earth);

    let mut simulator = NBodySimulator::new(GRAVITATIONAL_CONSTANT, FIXED_TIME_STEP, celestial_bodies);
    
    for _i in 1..125 {
        simulator.run_simulation_step(1);
    }
}