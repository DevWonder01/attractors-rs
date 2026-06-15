pub struct Lorenz {
    pub sigma: f32,
    pub rho: f32,
    pub beta: f32,
}

impl Lorenz {
    pub fn new(sigma: f32, rho: f32, beta: f32) -> Self {
        Self { sigma, rho, beta }
    }

    pub fn step(&mut self, x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
        let dx = self.sigma * (y - x) * dt;
        let dy = (x * (self.rho - z) - y) * dt;
        let dz = (x * y - self.beta * z) * dt;
        (x + dx, y + dy, z + dz)
    }
}
