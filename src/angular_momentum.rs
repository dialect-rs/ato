use std::fmt;
use num_enum::TryFromPrimitive;


/// Representation of the orbital angular momentum or the azimuthal quantum number.
/// The angular momentum can be created from the primitive value:
///
/// ```
///  use ato::angular_momentum::AngularMomentum;
///  let one = AngularMomentum::try_from(0u8);
///  assert_eq!(one, Ok(AngularMomentum::S));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
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

impl Hash for AngularMomentum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self as u8).hash(state);
    }
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
