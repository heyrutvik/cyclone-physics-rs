extern crate cyclone;
extern crate kiss3d;

use kiss3d::camera::Camera;
use kiss3d::camera::FirstPerson;
use kiss3d::nalgebra::{Point2, Point3, Translation3, Vector3};
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};

use cyclone::demo::timing::Timing;
use cyclone::particle::Particle;
use cyclone::precision::Real;
use kiss3d::event::{Action, Key, MouseButton, WindowEvent};
use kiss3d::text::Font;

#[derive(Debug, Clone, Copy)]
enum AmmoType {
    Pistol,
    Artillery,
    Fireball,
    Laser,
}

impl AmmoType {
    fn make_particle(&self) -> Particle {
        match self {
            AmmoType::Pistol => {
                let mut particle = Particle::new(0.0, 1.5, 0.0);
                particle.set_mass(2.0);
                particle.set_velocity(0.0, 0.0, 35.0);
                particle.set_acceleration(0.0, 0.0, 0.0);
                particle
            }
            AmmoType::Artillery => {
                let mut particle = Particle::new(0.0, 1.5, 0.0);
                particle.set_mass(200.0);
                particle.set_velocity(0.0, 30.0, 20.0);
                particle.set_acceleration(0.0, -20.0, 0.0);
                particle
            }
            AmmoType::Fireball => {
                let mut particle = Particle::new(0.0, 1.5, 0.0);
                particle.set_mass(1.0);
                particle.set_velocity(0.0, 0.0, 10.0);
                particle.set_acceleration(0.0, 2.0, 0.0);
                particle
            }
            AmmoType::Laser => {
                let mut particle = Particle::new(0.0, 1.5, 0.0);
                particle.set_mass(0.1);
                particle.set_velocity(0.0, 0.0, 150.0);
                particle.set_acceleration(0.0, 0.0, 0.0);
                particle
            }
        }
    }
}

struct Ammo {
    particle: Particle,
    scene_node: Option<SceneNode>,
}

impl Ammo {
    pub fn from_ammo_type(ammo_type: AmmoType) -> Ammo {
        let p = ammo_type.make_particle();
        Ammo {
            particle: p,
            scene_node: None,
        }
    }
}

struct Ballistic {
    ammo_type: AmmoType,
    magazine: Vec<Ammo>,
    camera: FirstPerson,
    timing: Timing,
}

impl Ballistic {
    fn init(&mut self, window: &mut Window) {
        // initialize graphics
        for n in 1..60 {
            let mut q = window.add_quad(10.0, 0.1, 1, 1);
            q.set_local_translation(Translation3::new(0.0, 0.0, n as Real));
        }
        let mut s = window.add_sphere(0.3);
        s.set_local_translation(Translation3::new(0.0, 1.5, 0.0));
        s.set_color(0.5, 0.5, 0.5);
    }
    fn handle_event(&mut self, window: &mut Window) {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Release, _) => {
                    if button == Key::Key1 {
                        self.ammo_type = AmmoType::Pistol;
                    } else if button == Key::Key2 {
                        self.ammo_type = AmmoType::Artillery;
                    } else if button == Key::Key3 {
                        self.ammo_type = AmmoType::Fireball;
                    } else if button == Key::Key4 {
                        self.ammo_type = AmmoType::Laser;
                    }
                }
                WindowEvent::MouseButton(button, Action::Release, _) => {
                    if button == MouseButton::Button1 {
                        self.fire();
                    }
                }
                _ => {}
            }
        }
    }
    fn render(&mut self, window: &mut Window) {
        // description of selected ammo type
        let font = Font::default();
        window.draw_text(
            "Click: Fire\n1-4: Select Ammo",
            &Point2::new(0.0, 0.0),
            60.0,
            &font,
            &Point3::new(0.0, 0.0, 0.0),
        );
        let current_ammo = format!("Current Ammo: {:?}", self.ammo_type);
        window.draw_text(
            &current_ammo,
            &Point2::new(0.0, 120.0),
            60.0,
            &font,
            &Point3::new(0.0, 0.0, 0.0),
        );

        // render fired ammo
        let mut removes = vec![];
        for (i, ammo) in self.magazine.iter_mut().enumerate() {
            let p = ammo.particle.get_position();
            let (x, y, z) = p.get_coordinates();
            if x > 0.0 || z < 60.0 {
                if let Some(ref mut sc) = ammo.scene_node {
                    sc.set_local_translation(Translation3::new(x, y, z));
                } else {
                    let mut s = window.add_sphere(0.3);
                    s.set_local_translation(Translation3::new(x, y, z));
                    s.set_color(1.0, 0.0, 0.0);
                    ammo.scene_node = Some(s);
                }
            } else if let Some(ref mut sc) = ammo.scene_node {
                sc.unlink();
                ammo.scene_node = None;
                removes.push(i);
            }
        }
        for idx in removes.iter() {
            self.magazine.remove(*idx);
        }
    }
    fn fire(&mut self) {
        // call on mouse click
        let bt = self.ammo_type;
        self.magazine.push(Ammo::from_ammo_type(bt));
    }
    fn update(&mut self) {
        self.timing.update(); // update game time
        for ammo in self.magazine.iter_mut() {
            // calculate ammo position
            ammo.particle.integrate(self.timing.get_duration());
        }
    }
}

impl State for Ballistic {
    fn step(&mut self, window: &mut Window) {
        self.handle_event(window);
        self.update();
        self.render(window);
    }
    fn cameras_and_effect_and_renderer(
        &mut self,
    ) -> (
        Option<&mut dyn Camera>,
        Option<&mut dyn PlanarCamera>,
        Option<&mut dyn Renderer>,
        Option<&mut dyn PostProcessingEffect>,
    ) {
        (Some(&mut self.camera), None, None, None)
    }
}

fn main() {
    let mut camera = FirstPerson::new_with_frustrum(
        45.0,
        1.0,
        500.0,
        Point3::new(-30.0, 5.0, -10.0),
        Point3::new(0.0, 5.0, 22.0),
    );
    camera.set_up_axis(Vector3::new(0.0, 1.0, 0.0));

    let timing = Timing::default();
    let ammo_type = AmmoType::Pistol;
    let magazine = vec![]; // empty magazine
    let mut demo = Ballistic {
        ammo_type,
        magazine,
        camera,
        timing,
    };

    let mut window = Window::new("Ballistic Demo");
    window.set_background_color(0.9, 0.95, 1.0);

    demo.init(&mut window);

    window.render_loop(demo);
}
