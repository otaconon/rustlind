use std::{
    io::{self, BufRead},
    str::FromStr,
};

pub struct Scanner<R> {
    reader: R,
    buf_str: String,
    pos: usize,
}

impl<R: BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: String::with_capacity(1024),
            pos: 0,
        }
    }

    pub fn next<T: FromStr>(&mut self) -> io::Result<T>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        loop {
            let unread = &self.buf_str[self.pos..];
            let trimmed = unread.trim_start_matches(|c: char| c.is_ascii_whitespace());

            self.pos += unread.len() - trimmed.len();
            if !trimmed.is_empty() {
                let token_len = trimmed
                    .find(|c: char| c.is_ascii_whitespace())
                    .unwrap_or(trimmed.len());

                let token = &trimmed[..token_len];
                self.pos += token_len;
                
                return token
                    .parse::<T>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
            }

            self.buf_str.clear();
            self.pos = 0;
            if self.reader.read_line(&mut self.buf_str)? == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Reached EOF while expecting another token",
                ));
            }
        }
    }
}
