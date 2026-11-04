pub enum Nucleobase {
    A, C, T, G, U
}

impl Nucleobase {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            b'A' => Some(Self::A),
            b'C' => Some(Self::C),
            b'G' => Some(Self::G),
            b'T' => Some(Self::T),
            _ => None,
        }
    }
}

pub struct NucleotideInfo {
    pub base: Nucleobase,
    pub count: u32,
    pub positions: Vec<usize>
}