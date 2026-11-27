use crate::amino_acid::AminoAcid;

pub struct Protein {
    seq: Vec<AminoAcid>
}

impl Protein {
    pub fn from_amino_acids(seq: Vec<AminoAcid>) -> Self {
        Self {
            seq
        } 
    }

    pub fn from_bytes<T: AsRef<[u8]>>(seq: T) -> Self {
        let parsed_seq = seq
            .as_ref()
            .iter()
            .copied()
            .filter_map(AminoAcid::from_byte)
            .collect();
        Self::from_amino_acids(parsed_seq)
    }

    pub fn seq(&self) -> &Vec<AminoAcid> {
        &self.seq
    }
}