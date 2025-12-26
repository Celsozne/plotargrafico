use std::fmt::Debug;

pub trait Resultado {
    fn calcular(&self, a: f64, b: f64) -> f64;
}

#[derive(Debug)]
pub struct Soma {}
impl Resultado for Soma {
    fn calcular(&self, a: f64, b: f64) -> f64 {
        a + b
    }
}

#[derive(Debug)]
pub struct Subtracao {}
impl Resultado for Subtracao {
    fn calcular(&self, a: f64, b: f64) -> f64 {
        a - b
    }
}

#[derive(Debug)]
pub struct Multiplicacao {}
impl Resultado for Multiplicacao {
    fn calcular(&self, a: f64, b: f64) -> f64 {
        a * b
    }
}

#[derive(Debug)]
pub struct Divisao {}
impl Resultado for Divisao {
    fn calcular(&self, a: f64, b: f64) -> f64 {
        if b == 0.0 { f64::NAN } else { a / b }
    }
}

#[derive(Debug)]
pub struct Potencia {}
impl Resultado for Potencia {
    fn calcular(&self, a: f64, b: f64) -> f64 {
        a.powf(b)
    }
}

#[derive(Debug)]
pub struct Raiz {}
impl Resultado for Raiz {
    fn calcular(&self, a: f64, b: f64) -> f64 {
        if b == 0.0 { f64::NAN } else { a.powf(1.0 / b) }
    }
}
