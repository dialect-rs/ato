use crate::angular_momentum::AngularMomentum;

#[derive(Debug, Clone)]
pub struct BasisFunction {
    pub l: AngularMomentum,
    pub exponents: Vec<f64>,
    pub coefficients: Vec<f64>,
}
