use crate::angular_momentum::AngularMomentum;
use crate::basis_function::BasisFunction;
use crate::bse::basisset_json::*;
use crate::bse::http::BasisSetExchange;
use crate::elements::Element;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BasisSet {
    pub name: String,
    pub description: String,
    pub basis_functions: HashMap<Element, Vec<BasisFunction>>,
}

impl From<InputData> for BasisSet {
    fn from(data: InputData) -> Self {
        // The HashMap is initialized.
        let mut bfs: HashMap<Element, Vec<BasisFunction>> = HashMap::new();
        for (element, shells) in data.elements.iter() {
            // The corresponding Element is created.
            let el: Element = Element::from(*element as u8);

            // The BasisFunctions are created.
            let mut functions: Vec<BasisFunction> = Vec::new();

            // Iteration over all shells.
            //for shell in shells.electron_shells.iter() {}
            let shell: &InputShell = &shells.electron_shells[shells.electron_shells.len() - 1];

            // The exponents are the same for all angular momenta.
            let exponents: Vec<f64> = shell
                .exponents
                .iter()
                .map(|x| x.parse::<f64>().unwrap())
                .collect();

            // Iteration over all angular momenta.
            for (l, c) in shell.angular_momentum.iter().zip(shell.coefficients.iter()) {
                // The coefficients are converted to floats.
                let coefficients: Vec<f64> = c.iter().map(|x| x.parse::<f64>().unwrap()).collect();

                // Add the new BasisFunction
                functions.push(BasisFunction {
                    l: AngularMomentum::from(*l),
                    exponents: exponents.clone(),
                    coefficients,
                });
            }

            bfs.insert(el, functions);
        }
        Self {
            name: data.name,
            description: data.description,
            basis_functions: bfs,
        }
    }
}

impl From<&str> for BasisSet {
    fn from(name: &str) -> Self {
        let data = BasisSetExchange::read_basis(name).unwrap();
        Self::from(data)
    }
}

impl BasisSet {
    /// Create a basis set.
    ///
    /// The names are identical to the ones listed at
    /// [BasisSetExchange](https://www.basissetexchange.org). If the library is called the first
    /// time it will download all basis sets from BSE and requires therefore an internet connection.
    /// The data of all basis sets have a size of about 400 MB and will be placed at
    /// $HOME/.ato_rs/data/basis_sets/
    /// If you want to change the path of the data directory you can set the following environment
    /// variable: $ATO_DATA_PATH
    pub fn new(name: &str) -> Self {
        Self::from(name)
    }

    /// Create a STO-3G basis set.
    ///
    /// This function serves merely as a convenience feature.
    pub fn sto3g() -> Self {
        Self::from("STO-3G")
    }

    pub fn repr_basis_set(&self, element: Element) -> String {
        let functions: &[BasisFunction] = self.basis_functions.get(&element).unwrap();
        let mut txt = "".to_owned();
        for function in functions.iter() {
            txt += &format!(
                " {}    {} {:1.2}\n",
                function.l,
                function.exponents.len(),
                1.00
            );
            for (e, c) in function.exponents.iter().zip(function.coefficients.iter()) {
                txt += &format!("{:18.14e} {:18.14e}\n", e, c);
            }
        }
        txt
    }
}

#[test]
fn test_sto3g() {
    let basis = BasisSet::sto3g();
    println!("{:?}", basis);
    assert_eq!(1, 2);
}
