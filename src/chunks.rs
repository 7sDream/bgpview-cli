use std::io::{self, Read, Write};

pub struct BodyChunks<'a> {
    chunks: Vec<&'a [u8]>,
    current_chunk: usize,
    current_pos: usize,
}

impl<'a> BodyChunks<'a> {
    pub fn new(chunks: Vec<&'a [u8]>) -> Self {
        Self {
            chunks,
            current_chunk: 0,
            current_pos: 0,
        }
    }
}

impl<'a> Read for BodyChunks<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut size = 0;
        let mut target = buf;
        loop {
            if self.current_chunk >= self.chunks.len() {
                break;
            } else if self.current_pos >= self.chunks[self.current_chunk].len() {
                break;
            } else {
                let (_, chunk) = self.chunks[self.current_chunk].split_at(self.current_pos);
                if target.len() > 0 {
                    let count = target.len().min(chunk.len());
                    size += target.write(&chunk[0..count])?;
                    if count == chunk.len() {
                        self.current_chunk += 1;
                        self.current_pos = 0;
                    } else {
                        self.current_pos += count;
                    }
                } else {
                    break;
                }
            }
        }
        Ok(size)
    }
}
