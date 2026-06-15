use crate::lorenz::Lorenz;
use egui_macroquad::egui;

pub struct Thomas {
    pub b: f32,
}

impl Thomas {
    pub fn new(b: f32) -> Self {
        Self { b }
    }

    pub fn step(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        let dx = (y.sin() - self.b * x) * dt;
        let dy = (z.sin() - self.b * y) * dt;
        let dz = (x.sin() - self.b * z) * dt;
        (x + dx, y + dy, z + dz)
    }
}

pub struct Chen {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Chen {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c }
    }
    pub fn step(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        let dx = (self.a * (y - x)) * dt;
        let dy = ((self.c - self.a) * x - x * z + self.c * y) * dt;
        let dz = (x * y - self.b * z) * dt;
        (x + dx, y + dy, z + dz)
    }
}

pub struct Dadras {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
}

impl Dadras {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32) -> Self {
        Self { a, b, c, d, e }
    }
    pub fn step(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        let dx = (y - self.a * x + self.b * y * z) * dt;
        let dy = (self.c * y - x * z + z) * dt;
        let dz = (self.d * x * y - self.e * z) * dt;
        (x + dx, y + dy, z + dz)
    }
}

pub struct Rossler {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Rossler {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c }
    }
    pub fn step(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        let dx = -(y + z) * dt;
        let dy = (x + self.a * y) * dt;
        let dz = (self.b + z * (x - self.c)) * dt;
        (x + dx, y + dy, z + dz)
    }
}

pub struct Halvorsen {
    pub a: f32,
}

impl Halvorsen {
    pub fn new(a: f32) -> Self {
        Self { a }
    }
    pub fn step(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        let dx = (-self.a * x - 4.0 * y - 4.0 * z - y * y) * dt;
        let dy = (-self.a * y - 4.0 * z - 4.0 * x - z * z) * dt;
        let dz = (-self.a * z - 4.0 * x - 4.0 * y - x * x) * dt;
        (x + dx, y + dy, z + dz)
    }
}

pub struct RabinovichFabrikant {
    pub alpha: f32,
    pub gamma: f32,
}

impl RabinovichFabrikant {
    pub fn new(alpha: f32, gamma: f32) -> Self {
        Self { alpha, gamma }
    }
    pub fn step(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        let dx = (y * (z - 1.0 + x * x) + self.gamma * x) * dt;
        let dy = (x * (3.0 * z + 1.0 - x * x) + self.gamma * y) * dt;
        let dz = (-2.0 * z * (self.alpha + x * y)) * dt;
        (x + dx, y + dy, z + dz)
    }
}

pub struct ThreeScroll {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl ThreeScroll {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        Self { a, b, c, d, e, f }
    }
    pub fn step(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        let dx = (self.a * (y - x) + self.d * x * z) * dt;
        let dy = (self.b * x - x * z + self.f * y) * dt;
        let dz = (self.c * z + x * y - self.e * x * x) * dt;
        (x + dx, y + dy, z + dz)
    }
}

pub struct Sprott {
    pub a: f32,
    pub b: f32,
}

impl Sprott {
    pub fn new(a: f32, b: f32) -> Self {
        Self { a, b }
    }
    pub fn step(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        let dx = (y + self.a * x * y + x * z) * dt;
        let dy = (1.0 - self.b * x * x + y * z) * dt;
        let dz = (x - x * x - y * y) * dt;
        (x + dx, y + dy, z + dz)
    }
}

pub struct FourWing {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl FourWing {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c }
    }
    pub fn step(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        let dx = (self.a * x + y * z) * dt;
        let dy = (self.b * x + self.c * y - x * z) * dt;
        let dz = (-z - x * y) * dt;
        (x + dx, y + dy, z + dz)
    }
}

#[derive(PartialEq)]
pub enum AttractorKind {
    Lorenz,
    Thomas,
    Chen,
    Dadras,
    Rossler,
    Halvorsen,
    RabinovichFabrikant,
    ThreeScroll,
    Sprott,
    FourWing,
}

pub enum Attractor {
    Lorenz(Lorenz),
    Thomas(Thomas),
    Chen(Chen),
    Dadras(Dadras),
    Rossler(Rossler),
    Halvorsen(Halvorsen),
    RabinovichFabrikant(RabinovichFabrikant),
    ThreeScroll(ThreeScroll),
    Sprott(Sprott),
    FourWing(FourWing),
}

impl Attractor {
    pub fn kind(&self) -> AttractorKind {
        match self {
            Attractor::Lorenz(_) => AttractorKind::Lorenz,
            Attractor::Thomas(_) => AttractorKind::Thomas,
            Attractor::Chen(_) => AttractorKind::Chen,
            Attractor::Dadras(_) => AttractorKind::Dadras,
            Attractor::Rossler(_) => AttractorKind::Rossler,
            Attractor::Halvorsen(_) => AttractorKind::Halvorsen,
            Attractor::RabinovichFabrikant(_) => AttractorKind::RabinovichFabrikant,
            Attractor::ThreeScroll(_) => AttractorKind::ThreeScroll,
            Attractor::Sprott(_) => AttractorKind::Sprott,
            Attractor::FourWing(_) => AttractorKind::FourWing,
        }
    }

    pub fn default_lorenz() -> Self { Attractor::Lorenz(Lorenz::new(10.0, 28.0, 8.0 / 3.0)) }
    pub fn default_thomas() -> Self { Attractor::Thomas(Thomas::new(0.208186)) }
    pub fn default_chen() -> Self { Attractor::Chen(Chen::new(35.0, 3.0, 28.0)) }
    pub fn default_dadras() -> Self { Attractor::Dadras(Dadras::new(3.0, 2.7, 1.7, 2.0, 9.0)) }
    pub fn default_rossler() -> Self { Attractor::Rossler(Rossler::new(0.2, 0.2, 5.7)) }
    pub fn default_halvorsen() -> Self { Attractor::Halvorsen(Halvorsen::new(1.89)) }
    pub fn default_rf() -> Self { Attractor::RabinovichFabrikant(RabinovichFabrikant::new(0.14, 0.10)) }
    pub fn default_threescroll() -> Self { Attractor::ThreeScroll(ThreeScroll::new(32.48, 45.84, 1.18, 0.13, 0.57, 14.7)) }
    pub fn default_sprott() -> Self { Attractor::Sprott(Sprott::new(2.07, 1.79)) }
    pub fn default_fourwing() -> Self { Attractor::FourWing(FourWing::new(0.2, 0.01, -0.4)) }

    pub fn step(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        match self {
            Attractor::Lorenz(a) => a.step(x, y, z, dt),
            Attractor::Thomas(a) => a.step(x, y, z, dt),
            Attractor::Chen(a) => a.step(x, y, z, dt),
            Attractor::Dadras(a) => a.step(x, y, z, dt),
            Attractor::Rossler(a) => a.step(x, y, z, dt),
            Attractor::Halvorsen(a) => a.step(x, y, z, dt),
            Attractor::RabinovichFabrikant(a) => a.step(x, y, z, dt),
            Attractor::ThreeScroll(a) => a.step(x, y, z, dt),
            Attractor::Sprott(a) => a.step(x, y, z, dt),
            Attractor::FourWing(a) => a.step(x, y, z, dt),
        }
    }

    pub fn draw_ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        match self {
            Attractor::Lorenz(a) => {
                changed |= ui.add(egui::Slider::new(&mut a.sigma, 0.0..=30.0).text("Sigma")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.rho, 0.0..=100.0).text("Rho")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.beta, 0.0..=10.0).text("Beta")).changed();
            }
            Attractor::Thomas(a) => {
                changed |= ui.add(egui::Slider::new(&mut a.b, 0.0..=1.0).text("b")).changed();
            }
            Attractor::Chen(a) => {
                changed |= ui.add(egui::Slider::new(&mut a.a, 0.0..=50.0).text("a")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.b, 0.0..=10.0).text("b")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.c, 0.0..=50.0).text("c")).changed();
            }
            Attractor::Dadras(a) => {
                changed |= ui.add(egui::Slider::new(&mut a.a, 0.0..=5.0).text("a")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.b, 0.0..=5.0).text("b")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.c, 0.0..=5.0).text("c")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.d, 0.0..=5.0).text("d")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.e, 0.0..=15.0).text("e")).changed();
            }
            Attractor::Rossler(a) => {
                changed |= ui.add(egui::Slider::new(&mut a.a, 0.0..=1.0).text("a")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.b, 0.0..=1.0).text("b")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.c, 0.0..=10.0).text("c")).changed();
            }
            Attractor::Halvorsen(a) => {
                changed |= ui.add(egui::Slider::new(&mut a.a, 0.0..=5.0).text("a")).changed();
            }
            Attractor::RabinovichFabrikant(a) => {
                changed |= ui.add(egui::Slider::new(&mut a.alpha, 0.0..=1.0).text("Alpha")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.gamma, 0.0..=1.0).text("Gamma")).changed();
            }
            Attractor::ThreeScroll(a) => {
                changed |= ui.add(egui::Slider::new(&mut a.a, 0.0..=50.0).text("a")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.b, 0.0..=50.0).text("b")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.c, 0.0..=5.0).text("c")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.d, 0.0..=1.0).text("d")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.e, 0.0..=1.0).text("e")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.f, 0.0..=20.0).text("f")).changed();
            }
            Attractor::Sprott(a) => {
                changed |= ui.add(egui::Slider::new(&mut a.a, 0.0..=5.0).text("a")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.b, 0.0..=5.0).text("b")).changed();
            }
            Attractor::FourWing(a) => {
                changed |= ui.add(egui::Slider::new(&mut a.a, 0.0..=1.0).text("a")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.b, 0.0..=1.0).text("b")).changed();
                changed |= ui.add(egui::Slider::new(&mut a.c, -1.0..=1.0).text("c")).changed();
            }
        }
        changed
    }
}
