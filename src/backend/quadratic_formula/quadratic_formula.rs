#[derive(Debug)]
pub struct QuadraticFormula {
    a: f64,
    b: f64,
    c: f64,
}

impl QuadraticFormula {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    pub fn calculate(&self) -> (f64, f64) {
        let d = self.b.powi(2) - 4.0 * self.a * self.c;
        let x1 = (-self.b + d.sqrt()) / (2.0 * self.a);
        let x2 = (-self.b - d.sqrt()) / (2.0 * self.a);
        (x1, x2)
    }
}