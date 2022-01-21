use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum AngularMomentum {
    S = 0,
    P = 1,
    D = 2,
    F = 3,
    G = 4,
    H = 5,
    I = 6,
    J = 7,
    K = 8,
    L = 9,
    M = 10,
}

impl fmt::Display for AngularMomentum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol: &str = match self {
            AngularMomentum::S => "s",
            AngularMomentum::P => "p",
            AngularMomentum::D => "d",
            AngularMomentum::F => "f",
            AngularMomentum::G => "g",
            AngularMomentum::H => "h",
            AngularMomentum::I => "i",
            AngularMomentum::J => "j",
            AngularMomentum::K => "k",
            AngularMomentum::L => "l",
            AngularMomentum::M => "m",
        };
        write!(f, "{}", symbol)
    }
}

impl From<usize> for AngularMomentum {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::S,
            1 => Self::P,
            2 => Self::D,
            3 => Self::F,
            4 => Self::G,
            5 => Self::H,
            6 => Self::I,
            7 => Self::J,
            8 => Self::K,
            9 => Self::L,
            10 => Self::M,
            a => {
                panic!("Angular momentum:{} is not implemented", a)
            }
        }
    }
}
