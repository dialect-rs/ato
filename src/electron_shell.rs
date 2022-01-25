use num_enum::TryFromPrimitive;

/// Representation of the principal quantum number or the electron shell.
/// The shell can be created from the primitive value:
///
/// ```
///  use ato::electron_shell::Shell;
///  let one = Shell::try_from(1u8);
///  assert_eq!(one, Ok(Shell::One));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Shell {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

