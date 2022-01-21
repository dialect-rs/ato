use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[rustfmt::skip]
#[derive(Clone, Copy,Debug)]
/// Type that contains the kind of a chemical element.
pub enum Element {
    H,                                                                                   He,
    Li,  Be,                                                    B,   C,   N,   O,   F,   Ne,
    Na,  Mg,                                                    Al,  Si,  P,   S,   Cl,  Ar,
    K,   Ca,  Sc,  Ti,  V,   Cr,  Mn,  Fe,  Co,  Ni,  Cu,  Zn,  Ga,  Ge,  As,  Se,  Br,  Kr,
    Rb,  Sr,  Y,   Zr,  Nb,  Mo,  Tc,  Ru,  Rh,  Pd,  Ag,  Cd,  In,  Sn,  Sb,  Te,  I,   Xe,
    Cs,  Ba,  Hf,  Ta,  W,   Re,  Os,  Ir,  Pt,  Au,  Hg,  Tl,  Pb,  Bi,  Po,  At,  Rn,
    Fr,  Ra,  Rf,  Db,  Sg,  Bh,  Hs,  Mt,  Ds,  Rg,  Cn,  Nh,  Fl,  Mc,  Lv,  Ts,  Og,

    La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu,
    Ac, Th, Pa, U,  Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr,
    Dummy,
}
// Two elements are considered to be the same if they have the same atomic number
impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.number() == other.number()
    }
}

impl Eq for Element {}

// Two elements are compared by their atomic numbers
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number().partial_cmp(&other.number())
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number().cmp(&other.number())
    }
}

// The atomic number of the chemical element is used for the hashing
impl Hash for Element {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number().hash(state);
    }
}

impl From<&str> for Element {
    fn from(symbol: &str) -> Self {
        match symbol.to_lowercase().as_str() {
            "h" => Element::H,
            "he" => Element::He,
            "li" => Element::Li,
            "be" => Element::Be,
            "b" => Element::B,
            "c" => Element::C,
            "n" => Element::N,
            "o" => Element::O,
            "f" => Element::F,
            "ne" => Element::Ne,
            "na" => Element::Na,
            "mg" => Element::Mg,
            "al" => Element::Al,
            "si" => Element::Si,
            "p" => Element::P,
            "s" => Element::S,
            "cl" => Element::Cl,
            "ar" => Element::Ar,
            "k" => Element::K,
            "ca" => Element::Ca,
            "sc" => Element::Sc,
            "ti" => Element::Ti,
            "v" => Element::V,
            "cr" => Element::Cr,
            "mn" => Element::Mn,
            "fe" => Element::Fe,
            "co" => Element::Co,
            "ni" => Element::Ni,
            "cu" => Element::Cu,
            "zn" => Element::Zn,
            "ga" => Element::Ga,
            "ge" => Element::Ge,
            "as" => Element::As,
            "se" => Element::Se,
            "br" => Element::Br,
            "kr" => Element::Kr,
            "rb" => Element::Rb,
            "sr" => Element::Sr,
            "y" => Element::Y,
            "zr" => Element::Zr,
            "nb" => Element::Nb,
            "mo" => Element::Mo,
            "tc" => Element::Tc,
            "ru" => Element::Ru,
            "rh" => Element::Rh,
            "pd" => Element::Pd,
            "ag" => Element::Ag,
            "cd" => Element::Cd,
            "in" => Element::In,
            "sn" => Element::Sn,
            "sb" => Element::Sb,
            "te" => Element::Te,
            "i" => Element::I,
            "xe" => Element::Xe,
            "cs" => Element::Cs,
            "ba" => Element::Ba,
            "la" => Element::La,
            "ce" => Element::Ce,
            "pr" => Element::Pr,
            "nd" => Element::Nd,
            "pm" => Element::Pm,
            "sm" => Element::Sm,
            "eu" => Element::Eu,
            "gd" => Element::Gd,
            "tb" => Element::Tb,
            "dy" => Element::Dy,
            "ho" => Element::Ho,
            "er" => Element::Er,
            "tm" => Element::Tm,
            "yb" => Element::Yb,
            "lu" => Element::Lu,
            "hf" => Element::Hf,
            "ta" => Element::Ta,
            "w" => Element::W,
            "re" => Element::Re,
            "os" => Element::Os,
            "ir" => Element::Ir,
            "pt" => Element::Pt,
            "au" => Element::Au,
            "hg" => Element::Hg,
            "tl" => Element::Tl,
            "pb" => Element::Pb,
            "bi" => Element::Bi,
            "po" => Element::Po,
            "at" => Element::At,
            "rn" => Element::Rn,
            "fr" => Element::Fr,
            "ra" => Element::Ra,
            "ac" => Element::Ac,
            "th" => Element::Th,
            "pa" => Element::Pa,
            "u" => Element::U,
            "np" => Element::Np,
            "pu" => Element::Pu,
            "am" => Element::Am,
            "cm" => Element::Cm,
            "bk" => Element::Bk,
            "cf" => Element::Cf,
            "es" => Element::Es,
            "fm" => Element::Fm,
            "md" => Element::Md,
            "no" => Element::No,
            "lr" => Element::Lr,
            "rf" => Element::Rf,
            "db" => Element::Db,
            "sg" => Element::Sg,
            "bh" => Element::Bh,
            "hs" => Element::Hs,
            "mt" => Element::Mt,
            "ds " => Element::Ds,
            "rg " => Element::Rg,
            "cn " => Element::Cn,
            "nh" => Element::Nh,
            "fl" => Element::Fl,
            "mc" => Element::Mc,
            "lv" => Element::Lv,
            "ts" => Element::Ts,
            "og" => Element::Og,
            _ => Element::Dummy,
        }
    }
}

impl From<u8> for Element {
    fn from(number: u8) -> Self {
        match number {
            1u8 => Element::H,
            2u8 => Element::He,
            3u8 => Element::Li,
            4u8 => Element::Be,
            5u8 => Element::B,
            6u8 => Element::C,
            7u8 => Element::N,
            8u8 => Element::O,
            9u8 => Element::F,
            10u8 => Element::Ne,
            11u8 => Element::Na,
            12u8 => Element::Mg,
            13u8 => Element::Al,
            14u8 => Element::Si,
            15u8 => Element::P,
            16u8 => Element::S,
            17u8 => Element::Cl,
            18u8 => Element::Ar,
            19u8 => Element::K,
            20u8 => Element::Ca,
            21u8 => Element::Sc,
            22u8 => Element::Ti,
            23u8 => Element::V,
            24u8 => Element::Cr,
            25u8 => Element::Mn,
            26u8 => Element::Fe,
            27u8 => Element::Co,
            28u8 => Element::Ni,
            29u8 => Element::Cu,
            30u8 => Element::Zn,
            31u8 => Element::Ga,
            32u8 => Element::Ge,
            33u8 => Element::As,
            34u8 => Element::Se,
            35u8 => Element::Br,
            36u8 => Element::Kr,
            37u8 => Element::Rb,
            38u8 => Element::Sr,
            39u8 => Element::Y,
            40u8 => Element::Zr,
            41u8 => Element::Nb,
            42u8 => Element::Mo,
            43u8 => Element::Tc,
            44u8 => Element::Ru,
            45u8 => Element::Rh,
            46u8 => Element::Pd,
            47u8 => Element::Ag,
            48u8 => Element::Cd,
            49u8 => Element::In,
            50u8 => Element::Sn,
            51u8 => Element::Sb,
            52u8 => Element::Te,
            53u8 => Element::I,
            54u8 => Element::Xe,
            55u8 => Element::Cs,
            56u8 => Element::Ba,
            57u8 => Element::La,
            58u8 => Element::Ce,
            59u8 => Element::Pr,
            60u8 => Element::Nd,
            61u8 => Element::Pm,
            62u8 => Element::Sm,
            63u8 => Element::Eu,
            64u8 => Element::Gd,
            65u8 => Element::Tb,
            66u8 => Element::Dy,
            67u8 => Element::Ho,
            68u8 => Element::Er,
            69u8 => Element::Tm,
            70u8 => Element::Yb,
            71u8 => Element::Lu,
            72u8 => Element::Hf,
            73u8 => Element::Ta,
            74u8 => Element::W,
            75u8 => Element::Re,
            76u8 => Element::Os,
            77u8 => Element::Ir,
            78u8 => Element::Pt,
            79u8 => Element::Au,
            80u8 => Element::Hg,
            81u8 => Element::Tl,
            82u8 => Element::Pb,
            83u8 => Element::Bi,
            84u8 => Element::Po,
            85u8 => Element::At,
            86u8 => Element::Rn,
            87u8 => Element::Fr,
            88u8 => Element::Ra,
            89u8 => Element::Ac,
            90u8 => Element::Th,
            91u8 => Element::Pa,
            92u8 => Element::U,
            93u8 => Element::Np,
            94u8 => Element::Pu,
            95u8 => Element::Am,
            96u8 => Element::Cm,
            97u8 => Element::Bk,
            98u8 => Element::Cf,
            99u8 => Element::Es,
            100u8 => Element::Fm,
            101u8 => Element::Md,
            102u8 => Element::No,
            103u8 => Element::Lr,
            104u8 => Element::Rf,
            105u8 => Element::Db,
            106u8 => Element::Sg,
            107u8 => Element::Bh,
            108u8 => Element::Hs,
            109u8 => Element::Mt,
            110u8 => Element::Ds,
            111u8 => Element::Rg,
            112u8 => Element::Cn,
            113u8 => Element::Nh,
            114u8 => Element::Fl,
            115u8 => Element::Mc,
            116u8 => Element::Lv,
            117u8 => Element::Ts,
            118u8 => Element::Og,
            _ => Element::Dummy,
        }
    }
}

impl Element {
    pub fn symbol(&self) -> &'static str {
        match *self {
            Element::Dummy => "X",
            Element::H => "H",
            Element::He => "He",
            Element::Li => "Li",
            Element::Be => "Be",
            Element::B => "B",
            Element::C => "C",
            Element::N => "N",
            Element::O => "O",
            Element::F => "F",
            Element::Ne => "Ne",
            Element::Na => "Na",
            Element::Mg => "Mg",
            Element::Al => "Al",
            Element::Si => "Si",
            Element::P => "P",
            Element::S => "S",
            Element::Cl => "Cl",
            Element::Ar => "Ar",
            Element::K => "K",
            Element::Ca => "Ca",
            Element::Sc => "Sc",
            Element::Ti => "Ti",
            Element::V => "V",
            Element::Cr => "Cr",
            Element::Mn => "Mn",
            Element::Fe => "Fe",
            Element::Co => "Co",
            Element::Ni => "Ni",
            Element::Cu => "Cu",
            Element::Zn => "Zn",
            Element::Ga => "Ga",
            Element::Ge => "Ge",
            Element::As => "As",
            Element::Se => "Se",
            Element::Br => "Br",
            Element::Kr => "Kr",
            Element::Rb => "Rb",
            Element::Sr => "Sr",
            Element::Y => "Y",
            Element::Zr => "Zr",
            Element::Nb => "Nb",
            Element::Mo => "Mo",
            Element::Tc => "Tc",
            Element::Ru => "Ru",
            Element::Rh => "Rh",
            Element::Pd => "Pd",
            Element::Ag => "Ag",
            Element::Cd => "Cd",
            Element::In => "In",
            Element::Sn => "Sn",
            Element::Sb => "Sb",
            Element::Te => "Te",
            Element::I => "I",
            Element::Xe => "Xe",
            Element::Cs => "Cs",
            Element::Ba => "Ba",
            Element::La => "La",
            Element::Ce => "Ce",
            Element::Pr => "Pr",
            Element::Nd => "Nd",
            Element::Pm => "Pm",
            Element::Sm => "Sm",
            Element::Eu => "Eu",
            Element::Gd => "Gd",
            Element::Tb => "Tb",
            Element::Dy => "Dy",
            Element::Ho => "Ho",
            Element::Er => "Er",
            Element::Tm => "Tm",
            Element::Yb => "Yb",
            Element::Lu => "Lu",
            Element::Hf => "Hf",
            Element::Ta => "Ta",
            Element::W => "W",
            Element::Re => "Re",
            Element::Os => "Os",
            Element::Ir => "Ir",
            Element::Pt => "Pt",
            Element::Au => "Au",
            Element::Hg => "Hg",
            Element::Tl => "Tl",
            Element::Pb => "Pb",
            Element::Bi => "Bi",
            Element::Po => "Po",
            Element::At => "At",
            Element::Rn => "Rn",
            Element::Fr => "Fr",
            Element::Ra => "Ra",
            Element::Ac => "Ac",
            Element::Th => "Th",
            Element::Pa => "Pa",
            Element::U => "U",
            Element::Np => "Np",
            Element::Pu => "Pu",
            Element::Am => "Am",
            Element::Cm => "Cm",
            Element::Bk => "Bk",
            Element::Cf => "Cf",
            Element::Es => "Es",
            Element::Fm => "Fm",
            Element::Md => "Md",
            Element::No => "No",
            Element::Lr => "Lr",
            Element::Rf => "Rf",
            Element::Db => "Db",
            Element::Sg => "Sg",
            Element::Bh => "Bh",
            Element::Hs => "Hs",
            Element::Mt => "Mt",
            Element::Ds => "Ds",
            Element::Rg => "Rg",
            Element::Cn => "Cn",
            Element::Nh => "Nh",
            Element::Fl => "Fl",
            Element::Mc => "Mc",
            Element::Lv => "Lv",
            Element::Ts => "Ts",
            Element::Og => "Og",
        }
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Element::Dummy => "Dummy",
            Element::H => "Hydrogen",
            Element::He => "Helium",
            Element::Li => "Lithium",
            Element::Be => "Beryllium",
            Element::B => "Boron",
            Element::C => "Carbon",
            Element::N => "Nitrogen",
            Element::O => "Oxygen",
            Element::F => "Fluorine",
            Element::Ne => "Neon",
            Element::Na => "Sodium",
            Element::Mg => "Magnesium",
            Element::Al => "Aluminum",
            Element::Si => "Silicon",
            Element::P => "Phosphorus",
            Element::S => "Sulfur",
            Element::Cl => "Chlorine",
            Element::Ar => "Argon",
            Element::K => "Potassium",
            Element::Ca => "Calcium",
            Element::Sc => "Scandium",
            Element::Ti => "Titanium",
            Element::V => "Vanadium",
            Element::Cr => "Chromium",
            Element::Mn => "Manganese",
            Element::Fe => "Iron",
            Element::Co => "Cobalt",
            Element::Ni => "Nickel",
            Element::Cu => "Copper",
            Element::Zn => "Zinc",
            Element::Ga => "Gallium",
            Element::Ge => "Germanium",
            Element::As => "Arsenic",
            Element::Se => "Selenium",
            Element::Br => "Bromine",
            Element::Kr => "Krypton",
            Element::Rb => "Rubidium",
            Element::Sr => "Strontium",
            Element::Y => "Yttrium",
            Element::Zr => "Zirconium",
            Element::Nb => "Niobium",
            Element::Mo => "Molybdenum",
            Element::Tc => "Technetium",
            Element::Ru => "Ruthenium",
            Element::Rh => "Rhodium",
            Element::Pd => "Palladium",
            Element::Ag => "Silver",
            Element::Cd => "Cadmium",
            Element::In => "Indium",
            Element::Sn => "Tin",
            Element::Sb => "Antimony",
            Element::Te => "Tellurium",
            Element::I => "Iodine",
            Element::Xe => "Xenon",
            Element::Cs => "Cesium",
            Element::Ba => "Barium",
            Element::La => "Lanthanum",
            Element::Ce => "Cerium",
            Element::Pr => "Praseodymium",
            Element::Nd => "Neodymium",
            Element::Pm => "Promethium",
            Element::Sm => "Samarium",
            Element::Eu => "Europium",
            Element::Gd => "Gadolinium",
            Element::Tb => "Terbium",
            Element::Dy => "Dysprosium",
            Element::Ho => "Holmium",
            Element::Er => "Erbium",
            Element::Tm => "Thulium",
            Element::Yb => "Ytterbium",
            Element::Lu => "Lutetium",
            Element::Hf => "Hafnium",
            Element::Ta => "Tantalum",
            Element::W => "Wolfram",
            Element::Re => "Rhenium",
            Element::Os => "Osmium",
            Element::Ir => "Iridium",
            Element::Pt => "Platinum",
            Element::Au => "Gold",
            Element::Hg => "Mercury",
            Element::Tl => "Thallium",
            Element::Pb => "Lead",
            Element::Bi => "Bismuth",
            Element::Po => "Polonium",
            Element::At => "Astatine",
            Element::Rn => "Radon",
            Element::Fr => "Francium",
            Element::Ra => "Radium",
            Element::Ac => "Actinium",
            Element::Th => "Thorium",
            Element::Pa => "Protactinium",
            Element::U => "Uranium",
            Element::Np => "Neptunium",
            Element::Pu => "Plutonium",
            Element::Am => "Americium",
            Element::Cm => "Curium",
            Element::Bk => "Berkelium",
            Element::Cf => "Californium",
            Element::Es => "Einsteinium",
            Element::Fm => "Fermium",
            Element::Md => "Mendelevium",
            Element::No => "Nobelium",
            Element::Lr => "Lawrencium",
            Element::Rf => "Rutherfordium",
            Element::Db => "Dubnium",
            Element::Sg => "Seaborgium",
            Element::Bh => "Bohrium",
            Element::Hs => "Hassium",
            Element::Mt => "Meitnerium",
            Element::Ds => "Darmstadtium ",
            Element::Rg => "Roentgenium",
            Element::Cn => "Copernicium",
            Element::Nh => "Nihonium",
            Element::Fl => "Flerovium",
            Element::Mc => "Moscovium",
            Element::Lv => "Livermorium",
            Element::Ts => "Tennessine",
            Element::Og => "Oganesson",
        }
    }

    pub fn number(&self) -> u8 {
        match *self {
            Element::Dummy => 0u8,
            Element::H => 1u8,
            Element::He => 2u8,
            Element::Li => 3u8,
            Element::Be => 4u8,
            Element::B => 5u8,
            Element::C => 6u8,
            Element::N => 7u8,
            Element::O => 8u8,
            Element::F => 9u8,
            Element::Ne => 10u8,
            Element::Na => 11u8,
            Element::Mg => 12u8,
            Element::Al => 13u8,
            Element::Si => 14u8,
            Element::P => 15u8,
            Element::S => 16u8,
            Element::Cl => 17u8,
            Element::Ar => 18u8,
            Element::K => 19u8,
            Element::Ca => 20u8,
            Element::Sc => 21u8,
            Element::Ti => 22u8,
            Element::V => 23u8,
            Element::Cr => 24u8,
            Element::Mn => 25u8,
            Element::Fe => 26u8,
            Element::Co => 27u8,
            Element::Ni => 28u8,
            Element::Cu => 29u8,
            Element::Zn => 30u8,
            Element::Ga => 31u8,
            Element::Ge => 32u8,
            Element::As => 33u8,
            Element::Se => 34u8,
            Element::Br => 35u8,
            Element::Kr => 36u8,
            Element::Rb => 37u8,
            Element::Sr => 38u8,
            Element::Y => 39u8,
            Element::Zr => 40u8,
            Element::Nb => 41u8,
            Element::Mo => 42u8,
            Element::Tc => 43u8,
            Element::Ru => 44u8,
            Element::Rh => 45u8,
            Element::Pd => 46u8,
            Element::Ag => 47u8,
            Element::Cd => 48u8,
            Element::In => 49u8,
            Element::Sn => 50u8,
            Element::Sb => 51u8,
            Element::Te => 52u8,
            Element::I => 53u8,
            Element::Xe => 54u8,
            Element::Cs => 55u8,
            Element::Ba => 56u8,
            Element::La => 57u8,
            Element::Ce => 58u8,
            Element::Pr => 59u8,
            Element::Nd => 60u8,
            Element::Pm => 61u8,
            Element::Sm => 62u8,
            Element::Eu => 63u8,
            Element::Gd => 64u8,
            Element::Tb => 65u8,
            Element::Dy => 66u8,
            Element::Ho => 67u8,
            Element::Er => 68u8,
            Element::Tm => 69u8,
            Element::Yb => 70u8,
            Element::Lu => 71u8,
            Element::Hf => 72u8,
            Element::Ta => 73u8,
            Element::W => 74u8,
            Element::Re => 75u8,
            Element::Os => 76u8,
            Element::Ir => 77u8,
            Element::Pt => 78u8,
            Element::Au => 79u8,
            Element::Hg => 80u8,
            Element::Tl => 81u8,
            Element::Pb => 82u8,
            Element::Bi => 83u8,
            Element::Po => 84u8,
            Element::At => 85u8,
            Element::Rn => 86u8,
            Element::Fr => 87u8,
            Element::Ra => 88u8,
            Element::Ac => 89u8,
            Element::Th => 90u8,
            Element::Pa => 91u8,
            Element::U => 92u8,
            Element::Np => 93u8,
            Element::Pu => 94u8,
            Element::Am => 95u8,
            Element::Cm => 96u8,
            Element::Bk => 97u8,
            Element::Cf => 98u8,
            Element::Es => 99u8,
            Element::Fm => 100u8,
            Element::Md => 101u8,
            Element::No => 102u8,
            Element::Lr => 103u8,
            Element::Rf => 104u8,
            Element::Db => 105u8,
            Element::Sg => 106u8,
            Element::Bh => 107u8,
            Element::Hs => 108u8,
            Element::Mt => 109u8,
            Element::Ds => 110u8,
            Element::Rg => 111u8,
            Element::Cn => 112u8,
            Element::Nh => 113u8,
            Element::Fl => 114u8,
            Element::Mc => 115u8,
            Element::Lv => 116u8,
            Element::Ts => 117u8,
            Element::Og => 118u8,
        }
    }

    pub fn number_usize(&self) -> usize {
        match *self {
            Element::Dummy => 0,
            Element::H => 1,
            Element::He => 2,
            Element::Li => 3,
            Element::Be => 4,
            Element::B => 5,
            Element::C => 6,
            Element::N => 7,
            Element::O => 8,
            Element::F => 9,
            Element::Ne => 10,
            Element::Na => 11,
            Element::Mg => 12,
            Element::Al => 13,
            Element::Si => 14,
            Element::P => 15,
            Element::S => 16,
            Element::Cl => 17,
            Element::Ar => 18,
            Element::K => 19,
            Element::Ca => 20,
            Element::Sc => 21,
            Element::Ti => 22,
            Element::V => 23,
            Element::Cr => 24,
            Element::Mn => 25,
            Element::Fe => 26,
            Element::Co => 27,
            Element::Ni => 28,
            Element::Cu => 29,
            Element::Zn => 30,
            Element::Ga => 31,
            Element::Ge => 32,
            Element::As => 33,
            Element::Se => 34,
            Element::Br => 35,
            Element::Kr => 36,
            Element::Rb => 37,
            Element::Sr => 38,
            Element::Y => 39,
            Element::Zr => 40,
            Element::Nb => 41,
            Element::Mo => 42,
            Element::Tc => 43,
            Element::Ru => 44,
            Element::Rh => 45,
            Element::Pd => 46,
            Element::Ag => 47,
            Element::Cd => 48,
            Element::In => 49,
            Element::Sn => 50,
            Element::Sb => 51,
            Element::Te => 52,
            Element::I => 53,
            Element::Xe => 54,
            Element::Cs => 55,
            Element::Ba => 56,
            Element::La => 57,
            Element::Ce => 58,
            Element::Pr => 59,
            Element::Nd => 60,
            Element::Pm => 61,
            Element::Sm => 62,
            Element::Eu => 63,
            Element::Gd => 64,
            Element::Tb => 65,
            Element::Dy => 66,
            Element::Ho => 67,
            Element::Er => 68,
            Element::Tm => 69,
            Element::Yb => 70,
            Element::Lu => 71,
            Element::Hf => 72,
            Element::Ta => 73,
            Element::W => 74,
            Element::Re => 75,
            Element::Os => 76,
            Element::Ir => 77,
            Element::Pt => 78,
            Element::Au => 79,
            Element::Hg => 80,
            Element::Tl => 81,
            Element::Pb => 82,
            Element::Bi => 83,
            Element::Po => 84,
            Element::At => 85,
            Element::Rn => 86,
            Element::Fr => 87,
            Element::Ra => 88,
            Element::Ac => 89,
            Element::Th => 90,
            Element::Pa => 91,
            Element::U => 92,
            Element::Np => 93,
            Element::Pu => 94,
            Element::Am => 95,
            Element::Cm => 96,
            Element::Bk => 97,
            Element::Cf => 98,
            Element::Es => 99,
            Element::Fm => 100,
            Element::Md => 101,
            Element::No => 102,
            Element::Lr => 103,
            Element::Rf => 104,
            Element::Db => 105,
            Element::Sg => 106,
            Element::Bh => 107,
            Element::Hs => 108,
            Element::Mt => 109,
            Element::Ds => 110,
            Element::Rg => 111,
            Element::Cn => 112,
            Element::Nh => 113,
            Element::Fl => 114,
            Element::Mc => 115,
            Element::Lv => 116,
            Element::Ts => 117,
            Element::Og => 118,
        }
    }
}
