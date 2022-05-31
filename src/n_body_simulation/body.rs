use crate::math::vector_3::*;

 pub trait Body {
    
    fn mass(&self) -> &f64;
    fn velocity(&self) -> &Vector3;
    fn position(&self) -> &Vector3;

    fn add_velocity(&mut self, force: Vector3);
    fn update_position(&mut self, time_step: f64);

    /**
     * Updates the velocity of this celestial body based on the values from all given other bodies 
     */
    fn update_velocity(&mut self, other_bodies: &[impl Body], gravitational_constant: f64, time_step: f64) {

        // Iterates the other celestial bodies an updates the current velocity 
        // based on the gravitational pull from each
        for o_body in other_bodies {
            // Check the body is not equal to this body
            if self.position() == o_body.position() {
                continue;
            }

            // Update the current velocity based of the other body
            let sqr_dist: f64 = o_body.position().sqr_magnitude(*self.position());
            let dist: f64 = sqr_dist.sqrt(); // Magnitude (distance between points)
            let force_dir: Vector3 = (*o_body.position() - *self.position()) / dist;
            let force: Vector3 = force_dir * gravitational_constant * *o_body.mass() / sqr_dist;

            self.add_velocity(force * time_step);
        }
    }
}