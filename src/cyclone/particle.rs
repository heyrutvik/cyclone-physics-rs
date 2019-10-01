use crate::core::Vector3;
use crate::precision::{Real, REAL_MAX};

pub struct Particle {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
    force_accumulated: Vector3,

    damping: Real,
    inverse_mass: Real
}

impl Particle {
    pub fn new(x: Real, y: Real, z: Real) -> Particle {
        Particle {
            position: Vector3::new(x, y, z),
            velocity: Vector3::origin(),
            acceleration: Vector3::origin(),
            force_accumulated: Vector3::origin(),
            damping: 1.0,
            inverse_mass: 1.0
        }
    }

    pub fn set_mass(&mut self, mass: Real) {
        assert_ne!(mass, 0.0);
        self.inverse_mass = 1.0 / mass;
    }
    pub fn set_velocity(&mut self, x: Real, y: Real, z: Real) {
        self.velocity.x = x;
        self.velocity.y = y;
        self.velocity.z = z;
    }
    pub fn set_acceleration(&mut self, x: Real, y: Real, z: Real) {
        self.acceleration.x = x;
        self.acceleration.y = y;
        self.acceleration.z = z;
    }
    pub fn get_mass(&self) -> Real {
        if self.inverse_mass == 0.0 {
            REAL_MAX
        } else {
            1.0 / self.inverse_mass
        }
    }
    pub fn set_damping(&mut self, d: Real) {
        self.damping = d;
    }
    pub fn has_finite_mass(&self) -> bool {
        self.inverse_mass >= 0.0
    }
    pub fn clear_accumulator(&mut self) {
        self.force_accumulated.clear();
    }
    pub fn integrate(&mut self, duration: Real) {
        if self.inverse_mass <= 0.0 { return () }
        if duration > 0.0 {
            self.position.add_scaled_vector(&self.velocity, duration);
            let resulting_acc = &mut self.acceleration;
            resulting_acc.add_scaled_vector(&self.force_accumulated, self.inverse_mass);
            self.velocity.add_scaled_vector(&resulting_acc, duration);
            self.velocity *= self.damping.powf(duration);
            self.clear_accumulator();
        }
    }
    pub fn get_position(&self) -> &Vector3 { &self.position }
}