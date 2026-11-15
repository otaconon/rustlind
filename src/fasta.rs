use std::io::{self, BufRead};

use crate::{Strand, StrandType};

pub struct Record {
    pub id: String,
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

    pub fn read_record(&mut self) -> io::Result<Option<Record>> {
        let buf = self.reader.fill_buf()?;
        if buf.is_empty() {
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

        let mut header = String::new();
        self.reader.read_line(&mut header)?;
        self.line_num += 1;
        let id = header.trim_end().strip_prefix('>').unwrap_or_default().to_string();

        let mut seq = Vec::new();
        loop {
            let peek = self.reader.fill_buf()?;
            if peek.is_empty() || peek[0] == b'>' {
                break;
            }
            self.reader.read_until(b'\n', &mut seq)?;
            self.line_num += 1;
            trim_newline(&mut seq);
        }

        Ok(Some(Record{id, strand: Strand::from_bytes(seq, StrandType::DNA)})) // TODO: Handle RNA?
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