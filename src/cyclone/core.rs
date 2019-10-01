use std::ops::{Add, AddAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use crate::precision::Real;

#[derive(Debug, PartialEq)]
pub struct Vector3 {
    pub x: Real,
    pub y: Real,
    pub z: Real
}

impl Vector3 {
    pub fn new(x: Real, y: Real, z: Real) -> Vector3 {
        Vector3 {x, y, z}
    }
    pub fn origin() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }
    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
    pub fn magnitude(&self) -> Real {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn square_magnitude(&self) -> Real {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn normalize(&mut self) {
        let l: Real = self.magnitude();
        if l > 0.0 {
            *self *= 1.0 / l
        }
    }
    pub fn add_scaled_vector(&mut self, vector: &Vector3, scale: Real) {
        self.x += vector.x * scale;
        self.y += vector.y * scale;
        self.z += vector.z * scale;
    }
    pub fn component_product(&self, vector: &Vector3) -> Vector3 {
        Vector3::new(self.x * vector.x, self.y * vector.y, self.z * vector.z)
    }
    pub fn component_product_update(&mut self, vector: &Vector3) {
        self.x *= vector.x;
        self.y *= vector.y;
        self.z *= vector.z;
    }
    pub fn make_orthonormal_basis(a: &mut Vector3, b: &mut Vector3, c: &mut Vector3) {
        a.normalize();
        *c = &*a % &*b;
        if c.square_magnitude() != 0.0 {
            c.normalize();
            *b = &*c % &*a;
        }
    }
}

impl Add<&Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<&Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign<&Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: &Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Real> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Real) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// scalar product *
impl Mul<&Vector3> for Vector3 {
    type Output = Real;
    fn mul(self, rhs: &Vector3) -> Self::Output {
        self.x*rhs.x + self.y*rhs.y + self.z+rhs.z
    }
}

impl MulAssign<Real> for Vector3 {
    fn mul_assign(&mut self, rhs: Real) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

// vector product %
impl Rem<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn rem(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(
            self.y*rhs.z - self.z*rhs.y,
            self.z*rhs.x - self.x*rhs.z,
            self.x*rhs.y - self.y*rhs.x)
    }
}

// vector product %=
impl RemAssign<&Vector3> for Vector3 {
    fn rem_assign(&mut self, rhs: &Vector3) {
        let x = self.y*rhs.z - self.z*rhs.y;
        let y = self.z*rhs.x - self.x*rhs.z;
        let z = self.x*rhs.y - self.y*rhs.x;
        self.x = x; self.y = y; self.z = z;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Vector3;
    use crate::precision::Real;

    #[test]
    fn test_invert() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        v1.invert();
        assert_eq!(v1, Vector3::new(-1.0, -2.0, -3.0));

        let mut v2 = Vector3::origin();
        v2.invert();
        assert_eq!(v2, Vector3::origin());
    }

    #[test]
    fn test_magnitue() {
        let mut v1 = Vector3::new(3.0,4.0,2.0);
        let mut sm1: Real = 29.0;
        assert_eq!(v1.square_magnitude(), sm1);
        assert_eq!(v1.magnitude(), sm1.sqrt());
    }

    #[test]
    fn test_normalize() {
        let mut v1 = Vector3::new(3.0,4.0,2.0);
        let sm1: Real = 29.0;
        let m1: Real = sm1.sqrt();
        let d1: Real = 1.0 / m1;
        v1.normalize();
        assert_eq!(v1, Vector3::new(3.0 * d1, 4.0 * d1, 2.0 * d1));
    }

    // TODO add more tests
}