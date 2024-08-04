use std::{cmp::min, ops::Sub, time::Duration};

use glam::Vec3;
use tracing::field::Field;

use crate::Instance;

pub enum FieldType {
    UField,
    VField,
    SField,
}

pub struct EulerSimulation {
    pub density: f32,
    pub width: usize,
    pub height: usize,
    pub cells_num: usize,
    pub instances: Vec<Instance>,

    u: Vec<f32>,
    v: Vec<f32>,
    pressure: Vec<f32>,
    solids: Vec<f32>,
    smoke: Vec<f32>,
    spacing: f32,
}

impl EulerSimulation {
    pub fn new(density: f32, width: usize, height: usize, spacing: f32) -> Self {
        let displacement = Vec3::new(1.0, 1.0, 0.0);

        let width = width + 2;
        let height = height + 2;

        let instances = (0..width)
            .flat_map(move |x| {
                (0..height).map(move |y| {
                    let position = Vec3::new(x as f32, y as f32, 0.0) - displacement;
                    let color = [1.0, 1.0, 1.0];
                    Instance { position, color }
                })
            })
            .collect::<Vec<_>>();

        let cells_num = width * height;

        let u: Vec<f32> = vec![0.0; cells_num];
        let v: Vec<f32> = vec![0.0; cells_num];
        let pressure: Vec<f32> = vec![0.0; cells_num];
        let solids: Vec<f32> = vec![0.0; cells_num];
        let smoke: Vec<f32> = vec![0.0; cells_num];

        Self {
            density,
            width,
            height,
            cells_num,
            instances,
            u,
            v,
            pressure,
            solids,
            smoke,
            spacing,
        }
    }

    pub fn init(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                if i == 0 || j == 0 || j == self.height - 1 {
                    self.solids[i * self.height + j] = 0.0; // solid
                    continue;
                } else {
                    self.solids[i * self.height + j] = 1.0;
                }

                if i > 51 && i < 69 && j > 40 && j < 60 {
                    self.solids[i * self.height + j] = 0.0; // solid
                    continue;
                }

                if i == 1 {
                    self.u[i * self.height + j] = 10.0;
                }
            }
        }
    }

    pub fn update(&mut self, dt: Duration) {
        let dt = dt.as_secs_f32();
        for i in 0..self.width {
            for j in 0..self.height {
                if j > 41 && j < 60 && i == 1 {
                    self.smoke[i * self.height + j] = 1.0;
                }
            }
        }

        self.pressure.fill(0.0);

        self.solve_incompressibility(20, dt);

        self.extrapolate();
        self.advect_vel(dt);
        self.advect_smoke(dt);

        let min_p = self.pressure.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_p = self
            .pressure
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b));

        for x in 0..self.width {
            for y in 0..self.height {
                let p = self.pressure[y * self.width + x];
                let s = self.smoke[y * self.width + x];
                let color = Self::get_color(p, min_p, max_p);
                let (r, g, b) = (
                    f32::max(0.0, color.0 * s),
                    f32::max(0.0, color.1 * s),
                    f32::max(0.0, color.2 * s),
                );

                // let velocity = self.v[(y * self.width + x) as usize];
                // self.instances[(y * self.width + x) as usize].color = [velocity, 0.0, 0.0];
                self.instances[(y * self.width + x) as usize].color = [r, g, b];
                // let smoke = self.smoke[(y * self.width + x) as usize];
                // self.instances[(y * self.width + x) as usize].color = [smoke, smoke, smoke];
            }
        }
    }

    fn get_color(val: f32, min_val: f32, max_val: f32) -> (f32, f32, f32) {
        let mut val = f32::min(f32::max(val, min_val), max_val - 0.01);
        let d = max_val - min_val;
        val = if d == 0.0 { 0.5 } else { (val - min_val) / d };
        let m = 0.25;
        let num = (val / m).floor();
        let s = (val - num * m) / m;
        let (mut r, mut g, mut b): (f32, f32, f32) = (0.0, 0.0, 0.0);
        match num as i32 {
            0 => {
                r = 0.0;
                g = s;
                b = 1.0;
            }
            1 => {
                r = 0.0;
                g = 1.0;
                b = 1.0 - s;
            }
            2 => {
                r = s;
                g = 1.0;
                b = 0.0;
            }
            3 => {
                r = 1.0;
                g = 1.0 - s;
                b = 0.0;
            }
            _ => {
                r = 0.0;
                g = 0.0;
                b = 0.0;
            }
        }

        return (r, g, b);
    }

    fn solve_incompressibility(&mut self, num_iters: usize, dt: f32) {
        let n = self.height;
        let cp = self.density * self.spacing / dt;

        for iteration in 0..num_iters {
            for i in 1..self.width - 1 {
                for j in 1..self.height - 1 {
                    if self.solids[i * n + j] == 0.0 {
                        continue;
                    }

                    let sx0 = self.solids[(i - 1) * n + j];
                    let sx1 = self.solids[(i + 1) * n + j];
                    let sy0 = self.solids[i * n + j - 1];
                    let sy1 = self.solids[i * n + j + 1];

                    let s = sx0 + sx1 + sy0 + sy1;

                    if s == 0.0 {
                        continue;
                    }

                    let div = self.u[(i + 1) * n + j] - self.u[i * n + j] + self.v[i * n + j + 1]
                        - self.v[i * n + j];

                    let mut p = -div / s;
                    p *= 1.9;
                    self.pressure[i * n + j] += cp * p;

                    self.u[i * n + j] -= sx0 * p;
                    self.u[(i + 1) * n + j] += sx1 * p;
                    self.v[i * n + j] -= sy0 * p;
                    self.v[i * n + j + 1] += sy1 * p;
                }
            }
        }
    }

    fn extrapolate(&mut self) {
        let n = self.height;
        for i in 0..self.width {
            self.u[i * n] = self.u[i * n + 1];
            self.u[i * n + self.height - 1] = self.u[i * n + self.height - 2];
        }

        for j in 0..self.height {
            self.v[j] = self.v[n + j];
            self.v[(self.width - 1) * n + j] = self.v[(self.width - 2) * n + 1];
        }
    }

    fn avg_u(&mut self, i: usize, j: usize) -> f32 {
        let n = self.height;
        let u = (self.u[i * n + j - 1]
            + self.u[i * n + j]
            + self.u[(i + 1) * n + j - 1]
            + self.u[(i + 1) * n + j])
            * 0.25;
        return u;
    }

    fn avg_v(&mut self, i: usize, j: usize) -> f32 {
        let n = self.height;
        let v = (self.v[(i - 1) * n + j]
            + self.v[i * n + j]
            + self.v[(i - 1) * n + j + 1]
            + self.v[i * n + j + 1])
            * 0.25;
        return v;
    }

    fn sample_field(&mut self, x: f32, y: f32, field_type: FieldType) -> f32 {
        let n = self.height;
        let h = self.spacing;
        let h1 = 1.0 / h;
        let h2 = 0.5 * h;

        let mut dx = 0.0;
        let mut dy = 0.0;

        let mut f;

        match field_type {
            FieldType::UField => {
                f = &self.u;
                dy = h2;
            }
            FieldType::VField => {
                f = &self.v;
                dx = h2;
            }
            FieldType::SField => {
                f = &self.smoke;
                dx = h2;
                dy = h2;
            }
        }

        let x0 = f32::min(((x - dx) * h1).floor(), (self.width - 1) as f32);
        let tx = ((x - dx) - x0 * h) * h1;
        let x1 = f32::min(x0 + 1.0, (self.width - 1) as f32);

        let y0 = f32::min(((y - dy) * h1).floor(), (self.height - 1) as f32);
        let ty = ((y - dy) - y0 * h) * h1;
        let y1 = f32::min(y0 + 1.0, (self.height - 1) as f32);

        let sx = 1.0 - tx;
        let sy = 1.0 - ty;

        let val = sx * sy * f[(x0 * n as f32 + y0) as usize]
            + tx * sy * f[(x1 * n as f32 + y0) as usize]
            + tx * ty * f[(x1 * n as f32 + y1) as usize]
            + sx * ty * f[(x0 * n as f32 + y1) as usize];

        return val;
    }

    fn advect_vel(&mut self, dt: f32) {
        let mut new_u = self.u.clone();
        let mut new_v = self.v.clone();

        let n = self.height;
        let h = self.spacing;
        let h2 = 0.5 * h;

        for i in 1..self.width {
            for j in 1..self.height {
                if self.solids[i * n + j] != 0.0
                    && self.solids[(i - 1) * n + j] != 0.0
                    && j < (self.height - 1)
                {
                    let mut x = i as f32 * h;
                    let mut y = j as f32 * h + h2;
                    let mut u = self.u[i * n + j];
                    let v = self.avg_v(i, j);
                    x = x - dt * u;
                    y = y - dt * v;
                    u = self.sample_field(x, y, FieldType::UField);
                    new_u[i * n + j] = u;
                }

                if self.solids[i * n + j] != 0.0
                    && self.solids[i * n + j - 1] != 0.0
                    && i < (self.width - 1)
                {
                    let mut x = i as f32 * h + h2;
                    let mut y = j as f32 * h;
                    let mut u = self.avg_u(i, j);
                    let mut v = self.v[i * n + j];
                    x = x - dt * u;
                    y = y - dt * v;
                    v = self.sample_field(x, y, FieldType::VField);
                    new_v[i * n + j] = v;
                }
            }
        }

        self.u = new_u;
        self.v = new_v;
    }

    fn advect_smoke(&mut self, dt: f32) {
        let mut new_smoke = self.smoke.clone();

        let n = self.height;
        let h = self.spacing;
        let h2 = 0.5 * h;

        for i in 0..self.width - 1 {
            for j in 0..self.height - 1 {
                if self.solids[i * n + j] != 0.0 {
                    let u = (self.u[i * n + j] + self.u[(i + 1) * n + j]) * 0.5;
                    let v = (self.v[i * n + j] + self.v[i * n + j + 1]) * 0.5;
                    let x = i as f32 * h + h2 - dt * u;
                    let y = j as f32 * h + h2 - dt * v;
                    new_smoke[i * n + j] = self.sample_field(x, y, FieldType::SField);
                }
            }
        }

        self.smoke = new_smoke;
    }
}
