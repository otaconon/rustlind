use crate::{Strand, StrandType};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodonStrand(Strand);

impl CodonStrand {
    pub fn strand(&self) -> &Strand {
        &self.0
    }
}

macro_rules! codons {
    ($($name:ident = $idx:expr => $letter:literal, [$($triplet:literal),+]),+ $(,)?) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Codon {
            $($name(CodonStrand)),+
        }

        impl Codon {
            pub const COUNT: usize = [$(stringify!($name)),+].len();

            pub fn from_strand(strand: Strand) -> Option<Self> {
                if strand.ty() != StrandType::RNA {
                    return None;
                }
                match strand.to_string().as_str() {
                    $($($triplet)|+ => Some(Codon::$name(CodonStrand(strand))),)+
                    _ => None,
                }
            }

            pub fn to_str(&self) -> &'static str {
                match self {
                    $(Codon::$name(_) => stringify!($name)),+
                }
            }

            pub fn letter(&self) -> char {
                match self {
                    $(Codon::$name(_) => $letter),+
                }
            }

            pub fn strand(&self) -> &Strand {
                match self {
                    $(Codon::$name(s) => &s.0),+
                }
            }

            pub fn index(&self) -> usize {
                match self {
                    $(Codon::$name(_) => $idx),+
                }
            }
        }
    };
}

codons! {
    Ala = 0  => 'A', ["GCU", "GCC", "GCA", "GCG"],
    Arg = 1  => 'R', ["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"],
    Asn = 2  => 'N', ["AAU", "AAC"],
    Asp = 3  => 'D', ["GAU", "GAC"],
    Cys = 4  => 'C', ["UGU", "UGC"],
    Gln = 5  => 'Q', ["CAA", "CAG"],
    Glu = 6  => 'E', ["GAA", "GAG"],
    Gly = 7  => 'G', ["GGU", "GGC", "GGA", "GGG"],
    His = 8  => 'H', ["CAU", "CAC"],
    Ile = 9  => 'I', ["AUU", "AUC", "AUA"],
    Leu = 10 => 'L', ["UUA", "UUG", "CUU", "CUC", "CUA", "CUG"],
    Lys = 11 => 'K', ["AAA", "AAG"],
    Met = 12 => 'M', ["AUG"],
    Phe = 13 => 'F', ["UUU", "UUC"],
    Pro = 14 => 'P', ["CCU", "CCC", "CCA", "CCG"],
    Ser = 15 => 'S', ["UCU", "UCC", "UCA", "UCG", "AGU", "AGC"],
    Thr = 16 => 'T', ["ACU", "ACC", "ACA", "ACG"],
    Trp = 17 => 'W', ["UGG"],
    Tyr = 18 => 'Y', ["UAU", "UAC"],
    Val = 19 => 'V', ["GUU", "GUC", "GUA", "GUG"],
    Stop = 20 => '*', ["UAA", "UAG", "UGA"],
}

#[derive(Debug, Clone, Default)]
pub struct CodonMap<T>(pub [T; Codon::COUNT]);

impl<T> Index<&Codon> for CodonMap<T> {
    type Output = T;

    #[inline]
    fn index(&self, codon: &Codon) -> &Self::Output {
        &self.0[codon.index()]
    }
}

impl<T> IndexMut<&Codon> for CodonMap<T> {
    #[inline]
    fn index_mut(&mut self, codon: &Codon) -> &mut Self::Output {
        &mut self.0[codon.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rna(s: &str) -> Strand {
        Strand::from_bytes(s, StrandType::RNA)
    }

    #[test]
    fn valid_codons() {
        assert!(matches!(Codon::from_strand(rna("AUG")), Some(Codon::Met(_))));
        assert!(matches!(Codon::from_strand(rna("AGC")), Some(Codon::Ser(_))));
    }

    #[test]
    fn letters() {
        assert_eq!(Codon::from_strand(rna("AUG")).unwrap().letter(), 'M');
        assert_eq!(Codon::from_strand(rna("AGC")).unwrap().letter(), 'S');
        assert_eq!(Codon::from_strand(rna("UGA")).unwrap().letter(), '*');
        assert_eq!(Codon::from_strand(rna("UGG")).unwrap().to_str(), "Trp");
    }

    #[test]
    fn invalid_codons() {
        assert!(Codon::from_strand(rna("AU")).is_none());
        assert!(Codon::from_strand(Strand::from_bytes("ATG", StrandType::DNA)).is_none());
    }
}
