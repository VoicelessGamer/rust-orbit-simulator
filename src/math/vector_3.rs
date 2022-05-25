
#[derive(Copy, Clone)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vector3 {
    fn new(x: f32, y: f32, z:f32) -> Vector3 {
        Vector3 {
            x: x, 
            y: y, 
            z: z
        }
    }

    /**
     * Returns the squared distance between this point and the right hand  side
     */
    pub fn sqr_magnitude(&self, _rhs: Vector3) -> f32 {
        let new_x = self.x - _rhs.x;
        let new_y = self.y - _rhs.y;
        let new_z = self.z - _rhs.z;

        return (new_x * new_x) + (new_y * new_y) + (new_z * new_z);
    }
}


/**
 * Overload for the subtract operator where right hand side is a Vector3
 * Returns a new Vector3 where the members variables are equal to the
 * matching right hand side variable is subtract by the matching left hand side variable
 */
impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: Vector3) -> Vector3 {

        Vector3 {
            x:self.x - _rhs.x,
            y:self.y - _rhs.y,
            z:self.z - _rhs.z
        }
    }
}

/**
 * Overload for the multiply operator where right hand side is a Vector3
 * Returns a new Vector3 where the members variables are equal to the
 * matching right hand side variable is multiplied by the matching left hand side variable
 */
impl std::ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Vector3 {

        Vector3 {
            x:self.x * _rhs.x,
            y:self.y * _rhs.y,
            z:self.z * _rhs.z
        }
    }
}

/**
 * Overload for the divide operator where right hand side is an f32
 * Returns a new Vector3 where each left hand side variable is divided by the f32
 */
impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f32) -> Vector3 {

        Vector3 {
            x:self.x / _rhs,
            y:self.y / _rhs,
            z:self.z / _rhs
        }
    }
}

/**
 * Overload for the multiply operator where right hand side is an f32
 * Returns a new Vector3 where each left hand side variable is multiplied by the f32
 */
impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f32) -> Vector3 {

        Vector3 {
            x:self.x * _rhs,
            y:self.y * _rhs,
            z:self.z * _rhs
        }
    }
}