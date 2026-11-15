use std::io::{self, BufRead};

use crate::{Strand, StrandType};

pub struct Record {
    pub id: Vec<u8>,
    pub strand: Strand,
}

pub struct Reader<R> {
    reader: R,
    line_num: usize,
}

impl<R: BufRead> Reader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            line_num: 0,
        }
    }

    pub fn read_record(&self) -> io::Result<Option<Record>> {
        let buf = self.reader.fill_buf()?;
        if buf.is_empty() {
            eprintln!("WARNING: Empty record on line {}", self.line_num);
            return Ok(None);
        }
        if buf[0] != b'>' {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Line {}: expected '>' at start of record",
                    self.line_num + 1
                ),
            ));
        }

        let mut record = Record{id: Vec::new(),strand: Strand::from_bytes(buf, StrandType::DNA)};
        self.reader.read_until(b'\n', &mut record.id)?;
        self.line_num += 1;
        record.id.remove(0); // Remove '>'
        trim_newline(&mut record.id);

        let seq = Vec::new();
        loop {
            let peek = self.reader.fill_buf()?;
            if peek.is_empty() || peek[0] == b'>' {
                break;
            }
            self.reader.read_until(b'\n', &mut seq)?;
            self.line_num += 1;
            trim_newline(&mut seq);
        }
        record.strand = Strand::from_bytes(seq, StrandType::DNA); // TODO: Handle RNA?

        Ok(Some(record))
    }
}

fn trim_newline(buf: &mut Vec<u8>) {
    if buf.ends_with(b"\n") {
        buf.pop();
        if buf.ends_with(b"\r") {
            buf.pop();
        }
    }
}