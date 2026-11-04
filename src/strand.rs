use crate::nucleotide::{Nucleobase, NucleotideInfo};
use crate::error::StrandError;

pub enum StrandType {
    RNA,
    DNA,
}

impl StrandType {
    fn infer(seq: &[u8]) -> Option<Self> {
        if seq.iter().all(|&b| matches!(b, b'A' | b'C' | b'G' | b'T')) {
            Some(Self::DNA)
        } else if seq.iter().all(|&b| matches!(b, b'A' | b'C' | b'G' | b'U')) {
            Some(Self::RNA)
        } else {
            None
        }
    }
}

pub struct Strand {
    pub seq: Vec<u8>,
    pub ty: StrandType,
    pub nucleotide_infos: Vec<NucleotideInfo>
}

impl Strand {
    pub fn new<T: Into<Vec<u8>>>(seq: T, ty: Option<StrandType>) -> Result<Self, StrandError> {
        let bytes: Vec<u8> = seq.into();
        let ty = match ty.or_else(|| StrandType::infer(&bytes)) {
            Some(ty) => ty,
            None => return Err(StrandError::InvalidStrand(bytes)),
        };

        const BASES: [Nucleobase; 5] = [
            Nucleobase::A, Nucleobase::C, Nucleobase::T, Nucleobase::G, Nucleobase::U,
        ];

        let mut positions: [Vec<usize>; 5] = Default::default();
        for (idx, &b) in bytes.iter().enumerate() {
            if let Some(base) = Nucleobase::from_byte(b) {
                positions[base as usize].push(idx);
            }
        }

        let nucleotide_infos: Vec<NucleotideInfo> = BASES
            .into_iter()
            .zip(positions)
            .map(|(base, positions)| NucleotideInfo {
                base,
                count: positions.len() as u32,
                positions,
            })
            .collect();

        Ok(Self {
            seq: bytes,
            ty,
            nucleotide_infos
        })
    }

    pub fn to_rna(&self) -> Result<Strand, StrandError> {
        let seq: Vec<u8> = self.seq.iter().map(|&b| if b == b'T' {b'U'} else {b}).collect();
        Strand::new(seq, Some(StrandType::RNA))
    }
}