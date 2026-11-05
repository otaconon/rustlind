use crate::StrandType;
use std::ops::{Index, IndexMut};

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Nucleobase {
    A = 0,
    T = 1,
    C = 2,
    G = 3,
    U = 4,
}

impl Nucleobase {
    pub const COUNT: usize = 5;
    pub const BASES: [Self; Self::COUNT] = [Self::A, Self::T, Self::C, Self::G, Self::U];

    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            b'A' => Some(Nucleobase::A),
            b'C' => Some(Nucleobase::C),
            b'G' => Some(Nucleobase::G),
            b'T' => Some(Nucleobase::T),
            b'U' => Some(Nucleobase::U),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Nucleobase::A => 'A',
            Nucleobase::T => 'T',
            Nucleobase::U => 'U',
            Nucleobase::C => 'C',
            Nucleobase::G => 'G',
        }
    }

    pub fn to_complement(&self, strand_type: StrandType) -> Nucleobase {
        match self {
            Nucleobase::A => {
                if strand_type == StrandType::DNA {
                    Nucleobase::T
                } else {
                    Nucleobase::U
                }
            }
            Nucleobase::T => Nucleobase::A,
            Nucleobase::U => Nucleobase::A,
            Nucleobase::C => Nucleobase::G,
            Nucleobase::G => Nucleobase::C,
        }
    }

    pub fn is_valid_for(&self, strand_type: StrandType) -> bool {
        return (*self != Nucleobase::T || strand_type == StrandType::DNA)
            && (*self != Nucleobase::U || strand_type == StrandType::RNA);
    }
}

#[derive(Debug, Clone, Default)]
pub struct NucleobaseMap<T>(pub [T; Nucleobase::COUNT]);

impl<T> Index<Nucleobase> for NucleobaseMap<T> {
    type Output = T;

    #[inline]
    fn index(&self, base: Nucleobase) -> &Self::Output {
        &self.0[base as usize]
    }
}

impl<T> IndexMut<Nucleobase> for NucleobaseMap<T> {
    #[inline]
    fn index_mut(&mut self, base: Nucleobase) -> &mut Self::Output {
        &mut self.0[base as usize]
    }
}

#[derive(Clone)]
pub struct NucleotideInfo {
    pub base: Nucleobase,
    pub positions: Vec<usize>,
}
