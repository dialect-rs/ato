use hashbrown::HashMap;
use crate::basis_function::BasisFunction;

#[derive(Serialize, Deserialize, Debug)]
struct InputData {
    name: String,
    description: String,
    elements: HashMap<usize, InputElement>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputElement {
    electron_shells: Vec<InputShell>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputShell {
    angular_momentum: Vec<usize>,
    exponents: Vec<String>,
    coefficients: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct BasisSet {
    name: String,
    description: String,
    basis_functions: HashMap<Element, Vec<BasisFunction>>,
}
