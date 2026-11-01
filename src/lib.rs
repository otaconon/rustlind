use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::env;

pub struct FileIO {
    pub reader: BufReader<File>,
    pub writer: BufWriter<Box<dyn Write>>
}

impl FileIO {
    pub fn from_args() -> io::Result<Self> {
        let mut args = env::args().skip(1);

        let input_path = args.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "No input path provided")
        })?;
        let reader = File::open(input_path)?;

        let raw_writer: Box<dyn Write> = match args.next() {
            Some(output_path) => Box::new(File::create(output_path)?),
            None => Box::new(io::stdout()),
        };

        Ok(Self {
            reader: BufReader::new(reader),
            writer: BufWriter::new(raw_writer),
        })
    }

    pub fn read(&mut self) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.reader.read_to_end(&mut buf)?;
        Ok(buf)
    }
}