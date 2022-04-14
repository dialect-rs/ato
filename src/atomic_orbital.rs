use crate::angular_momentum::AngularMomentum;
use crate::electron_shell::Shell;
use std::cmp::Ordering;
use std::convert::TryFrom;

/// Type that specifies an atomic orbital by its three quantum numbers and holds its energy
#[derive(Clone, Debug)]
pub struct AtomicOrbital {
    pub n: Shell,
    pub l: AngularMomentum,
    pub m: i8,
    pub energy: f64,
    pub exponents: Vec<f64>,
    pub coefficients: Vec<f64>,
}

impl AtomicOrbital {
    pub fn create_1s() -> Self {
        (1, 0, 0).into()
    }

    pub fn create_2s() -> Self {
        (2, 0, 0).into()
    }

    pub fn create_2px() -> Self {
        (2, 1, 1).into()
    }

    pub fn create_2py() -> Self {
        (2, 1, -1).into()
    }

    pub fn create_2pz() -> Self {
        (2, 1, 0).into()
    }

    pub fn create_3dxy() -> Self {
        (3, 2, -2).into()
    }

    pub fn create_3dyz() -> Self {
        (3, 2, -1).into()
    }

    pub fn create_3dz2() -> Self {
        (3, 2, 0).into()
    }

    pub fn create_3dxz() -> Self {
        (3, 2, 1).into()
    }

    pub fn create_3dx2y2() -> Self {
        (3, 2, 2).into()
    }
}

impl From<(i8, i8, i8)> for AtomicOrbital {
    fn from(qnumber: (i8, i8, i8)) -> Self {
        Self {
            n: Shell::try_from(qnumber.0 as u8).unwrap(),
            l: AngularMomentum::try_from(qnumber.1 as u8).unwrap(),
            m: qnumber.2,
            energy: 0.0,
            exponents: Vec::new(),
            coefficients: Vec::new(),
        }
    }
}

impl From<((i8, i8, i8), f64)> for AtomicOrbital {
    fn from(numbers_energy: ((i8, i8, i8), f64)) -> Self {
        Self {
            n: Shell::try_from(numbers_energy.0 .0 as u8).unwrap(),
            l: AngularMomentum::try_from(numbers_energy.0 .1 as u8).unwrap(),
            m: numbers_energy.0 .2,
            energy: numbers_energy.1,
            exponents: Vec::new(),
            coefficients: Vec::new(),
        }
    }
}

impl PartialOrd for AtomicOrbital {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ord_idx().partial_cmp(&other.ord_idx())
    }
}

impl AtomicOrbital {
    /// Compute an index, that orders the AtomicOrbital in Cartesian order.
    /// 1 s  => 100 |
    /// 2 s  => 200 | 2 px  => 210 | 2 py  => 211 | 2  pz => 212 |
    /// 3 s  => 300 | 3 px  => 310 | 3 py  => 311 | 3  pz => 312 |
    ///             | 3 dz2 => 320 | 3 dzx => 321 | 3 dyz => 322 | 3 dx2y2 => 323 | 3 dxy => 324
    pub fn ord_idx(&self) -> usize {
        let mut index: usize = self.n as usize * 100;
        index += self.l as usize * 10;
        index += match self.l {
            AngularMomentum::S => 0, // s-orbitals
            AngularMomentum::P => {
                // p-orbitals
                match self.m {
                    1 => 0,  // px
                    -1 => 1, // py
                    0 => 2,  // pz
                    _ => 3,
                }
            }
            AngularMomentum::D => {
                // d-orbitals
                match self.m {
                    0 => 0,  // dz2
                    1 => 1,  // dzx
                    -1 => 2, // dyz
                    -2 => 3, // dx2y2
                    2 => 4,  // dxy
                    _ => 5,
                }
            }
            _ => 6,
        };
        index
    }
}

impl PartialEq for AtomicOrbital {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n && self.m == other.m && self.l == other.l
    }
}

impl Eq for AtomicOrbital {}
