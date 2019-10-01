extern crate cyclone;
extern crate kiss3d;

use kiss3d::camera::Camera;
use kiss3d::camera::FirstPerson;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use nalgebra::{Point3, Translation3};

use cyclone::demo::timing::Timing;
use cyclone::particle::Particle;

struct Ball {
    particle: Particle,
    particle_node: Option<SceneNode>,
}

impl Ball {
    pub fn from_particle(p: Particle) -> Ball {
        Ball {particle: p, particle_node: None }
    }
}

struct BallSim {
    ball: Ball,
    camera: FirstPerson,
    timing: Timing
}

impl BallSim {
    fn render(&mut self, window: &mut Window) {
        let p = self.ball.particle.get_position();
        if let Some(ref mut sc) = self.ball.particle_node {
            sc.set_local_translation(Translation3::new(p.x, p.y, p.z));
        } else {
            let mut s = window.add_sphere(0.4);
            s.set_local_translation(Translation3::new(p.x, p.y, p.z));
            s.set_color(1.0, 0.0, 0.0);
            self.ball.particle_node = Some(s);
        }
    }
}

impl State for BallSim {
    fn step(&mut self, window: &mut Window) {
        self.timing.update();
        self.ball.particle.integrate(self.timing.get_duration());
        self.render(window);
    }
    fn cameras_and_effect_and_renderer(&mut self) -> (
        Option<&mut dyn Camera>,
        Option<&mut dyn PlanarCamera>,
        Option<&mut dyn Renderer>,
        Option<&mut dyn PostProcessingEffect>,
    ) {
        (Some(&mut self.camera), None, None, None)
    }
}

fn main() {
    let mut particle = Particle::new(0.0, 0.0, 0.0);
    particle.set_mass(1.0); // 1kg
    particle.set_velocity(15.0, 10.0, -10.0); // 18m/s
    particle.set_acceleration(0.0, -20.0, 0.0); // fall down

    let camera = FirstPerson::new(Point3::new(20.0, 20.0, 20.0), Point3::new(20.0, 19.0, 19.0));
    let timing = Timing::new();
    let ball = Ball::from_particle(particle);

    let sim = BallSim { ball, camera, timing };

    let mut window = Window::new("Ballistic Demo");
    window.set_background_color(0.9, 0.95, 1.0);
    window.set_framerate_limit(Some(60));
    window.render_loop(sim);
}