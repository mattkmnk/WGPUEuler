use std::{f64::consts::PI, time::Duration};

use glam::{DVec2, Vec3};
use rand::random;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::Instance;

const H: f64 = 16.0;
const HSQ: f64 = H * H;
const DT: f64 = 0.0001;
pub const G: DVec2 = DVec2::from_array([0.0, -9.81]);

#[derive(Debug, Default)]
pub struct SPHSimulation {
    pub width: f64,
    pub height: f64,
    pub instances: Vec<Instance>,

    pub max_particles: usize,
    pub num_particles: usize,
    pub position: Vec<DVec2>,
    velocity: Vec<DVec2>,
    forces: Vec<DVec2>,
    rho: Vec<f64>,
    pressure: Vec<f64>,
    mass: Vec<f64>,
}

impl SPHSimulation {
    pub fn new(width: f64, height: f64, max_particles: usize) -> Self {
        let instances = Vec::with_capacity(max_particles);
        let position = Vec::with_capacity(max_particles);
        let velocity = Vec::with_capacity(max_particles);
        let forces = Vec::with_capacity(max_particles);
        let rho = Vec::with_capacity(max_particles);
        let pressure = Vec::with_capacity(max_particles);
        let mass = Vec::with_capacity(max_particles);

        SPHSimulation {
            width,
            height,
            instances,
            max_particles,
            num_particles: 0,
            position,
            velocity,
            forces,
            rho,
            pressure,
            mass,
        }
    }

    pub fn init(&mut self) {
        self.init_scene(4096);
    }

    pub fn update(&mut self, dt: Duration) {
        self.compute_d_p();
        self.compute_forces();
        self.integrate(dt);
        self.update_instances();
    }

    pub fn update_instances(&mut self) {
        let mut instances: Vec<Instance> = Vec::with_capacity(self.max_particles);

        let mut i = 0;
        for particle in &self.position {
            let position = Vec3::new(particle[0] as f32, particle[1] as f32, 0.0);
            let color = [0.0, 0.0, 1.0];
            instances.push(Instance { position, color });
            i += 1;
        }

        self.instances = instances;
    }

    pub fn add_particle(&mut self, x: f64, y: f64) {
        self.num_particles += 1;
        self.position.push(DVec2::new(x, y));
        self.velocity.push(DVec2::ZERO);
        self.forces.push(DVec2::ZERO);
        self.rho.push(1.0);
        self.pressure.push(0.0);
        self.mass.push(1.0);
    }

    pub fn init_scene(&mut self, dam_max_particles: usize) {
        let mut placed = 0;
        let mut y = H;
        'outer: while y < 640.0 {
            y += H;
            let mut x = 640.0;
            while x <= 1280.0 {
                x += H;
                if placed == dam_max_particles || self.num_particles == self.max_particles {
                    break 'outer;
                }
                let jitter = random::<f64>();
                self.add_particle
        (x + jitter, y);
                placed += 1;
            }
        }
    }

    pub fn integrate(&mut self, dt: Duration) {
        self.position
            .par_iter_mut()
            .zip_eq(self.velocity.par_iter_mut())
            .enumerate()
            .for_each(|(i, (position, velocity))| {
                *velocity += DT * self.forces[i] / self.rho[i];
                *position += DT * *velocity;

                if position.x - H < 0.0 {
                    velocity.x *= -0.5;
                    position.x = H;
                }
                if position.x + H > self.width {
                    velocity.x *= -0.5;
                    position.x = self.width - H;
                }
                if position.y - H < 0.0 {
                    velocity.y *= -0.5;
                    position.y = H;
                }
                if position.y + H > self.height {
                    velocity.y *= -0.5;
                    position.y = self.height - H;
                }
            });
    }

    pub fn compute_d_p(&mut self) {
        let poly6 = 4.0 / (PI * f64::powf(H, 8.0));

        self.rho.par_iter_mut()
            .zip_eq(self.pressure.par_iter_mut())
            .enumerate()
            .for_each(|(i,(rho, pressure))| {
                *rho = 0.0;
                for j in 0..self.num_particles {
                    let pos_diff = self.position[j] - self.position[i];
                    let r = pos_diff.length_squared();
                    if r < HSQ {
                        *rho += self.mass[i] * poly6 * f64::powf(HSQ - r, 3.0);
                    }
                }
                *pressure = 3000.0 * (*rho - 1000.0);
            });
    }

    pub fn compute_forces(&mut self) {
        let spiky = -10.0 / (PI * f64::powf(H, 5.0));
        let viscy = 40.0 / (PI * f64::powf(H, 5.0));
        self.forces.par_iter_mut().enumerate()
            .for_each(|(i, forces)| {
                let mut fpress = DVec2::ZERO;
                let mut fvisc = DVec2::ZERO;
                for j in 0..self.num_particles {
                    if i == j {
                        continue;
                    }
                    let pos_diff = self.position[j] - self.position[i];
                    let dist: f64 = pos_diff.length();
                    if dist < H {
                        fpress += -pos_diff.normalize() * self.mass[i] * (self.pressure[i] + self.pressure[j])
                            / (2.0 * self.rho[j])
                            * spiky
                            * f64::powf(H - dist, 3.0);
                        fvisc += 100.0 * self.mass[i] * (self.velocity[j] - self.velocity[i]) / self.rho[j]
                            * viscy
                            * (H - dist);
                    }
                }
                let fgrav = G * self.mass[i] / self.rho[i];
                *forces = fpress + fvisc + fgrav;
            });
    }
}