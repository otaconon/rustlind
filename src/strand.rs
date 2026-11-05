use crate::StrandType;
use crate::{Nucleobase, NucleobaseMap, NucleotideInfo};
use std::fmt::{self, Write};

pub struct Strand {
    seq: Vec<Nucleobase>,
    ty: StrandType,
    nucleotide_infos: NucleobaseMap<NucleotideInfo>,
}

impl Strand {
    pub fn seq(&self) -> &Vec<Nucleobase> {
        &self.seq
    }
    pub fn ty(&self) -> StrandType {
        self.ty
    }
    pub fn nucleotide_infos(&self) -> &NucleobaseMap<NucleotideInfo> {
        &self.nucleotide_infos
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
        let mut nucleotide_infos = NucleobaseMap(Nucleobase::BASES.map(|base| NucleotideInfo {
            base,
            positions: Vec::new(),
        }));

        for (idx, &base) in seq.iter().enumerate() {
            let info = &mut nucleotide_infos[base];
            info.positions.push(idx);
        }

        Self {
            seq,
            ty,
            nucleotide_infos,
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
}

impl fmt::Display for Strand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &base in &self.seq {
            f.write_char(base.to_char())?;
        }
        Ok(())
    }
}
