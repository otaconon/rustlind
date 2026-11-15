use crate::StrandType;
use crate::{Nucleobase, NucleobaseMap};
use std::fmt::{self, Write};

pub struct Strand {
    seq: Vec<Nucleobase>,
    ty: StrandType,
}

impl Strand {
    pub fn seq(&self) -> &Vec<Nucleobase> {
        &self.seq
    }
    pub fn ty(&self) -> StrandType {
        self.ty
    }

    /// Creates strand from bytes.
    /// Skips any bytes that dont match nucleobases of provided StrandType
    pub fn from_bytes<T: AsRef<[u8]>>(seq: T, ty: StrandType) -> Self {
        let parsed_seq = seq
            .as_ref()
            .iter()
            .copied()
            .filter_map(Nucleobase::from_byte)
            .filter(|&base| base.is_valid_for(ty))
            .collect();

        Self::from_sequence(parsed_seq, ty)
    }

    pub fn from_sequence(seq: Vec<Nucleobase>, ty: StrandType) -> Self {
        Self {
            seq,
            ty,
        }
    }

    pub fn to_rna(&self) -> Strand {
        let seq: Vec<Nucleobase> = self
            .seq
            .iter()
            .map(|&b| if b == Nucleobase::T { Nucleobase::U } else { b })
            .collect();
        Strand::from_sequence(seq, StrandType::RNA)
    }

    pub fn to_complement(&self) -> Strand {
        let seq: Vec<Nucleobase> = self.seq.iter().map(|&b| b.to_complement(self.ty)).collect();
        Strand::from_sequence(seq, self.ty)
    }

    pub fn count(&self) -> NucleobaseMap<u32> {
        let mut count = NucleobaseMap([0; Nucleobase::COUNT]);
        for &base in &self.seq {
            count[base] += 1;
        }
        count
    }

    pub fn gc_content(&self) -> f64 {
        let c = self.count();
        (c[Nucleobase::G] + c[Nucleobase::C]) as f64 / self.seq.len() as f64
    }
}

impl fmt::Display for Strand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &base in &self.seq {
            f.write_char(base.to_char())?;
        }
        Ok(())
    }
}
