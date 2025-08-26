use std::{borrow::Borrow, io::{Read, Write}};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    key_length: usize,
    key_idx: usize,
}

struct XorcismReader<'a, R> {
    reader: R,
    xorcism: Xorcism<'a>
}

impl <'a, R:Read> Read for XorcismReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let tmp = self.reader.read(buf);
        if tmp.is_ok() {
            self.xorcism.munge_in_place(buf);
        }
        tmp
    }
}

struct XorcismWriter<'a, W> {
    writer: W,
    xorcism: Xorcism<'a>
}

impl <'a, W:Write> Write for XorcismWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let tmp: Vec<u8> = self.xorcism.munge(buf).collect();
        self.writer.write(&tmp)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        Xorcism { key: key.as_ref(), key_length: key.as_ref().len(), key_idx: 0 }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let mut i:usize = 0;
        let data_len = data.len();
        while i < data_len {
            data[i] ^= self.get_key_byte();
            i += 1;
        }
    }

    fn get_key_byte(&mut self) -> u8 {
        let byte = self.key[self.key_idx];
        if self.key_idx == (self.key_length - 1) {
            self.key_idx = 0;
        } else {
            self.key_idx += 1;
        }
        byte
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<T: Borrow<u8>, Data: IntoIterator<Item = T>>(&mut self, data: Data) -> impl Iterator<Item = u8> {
        data.into_iter().map(|byte| byte.borrow() ^ self.get_key_byte())
    }

    pub fn reader<R: Read>(self, r: R) -> impl Read {
        XorcismReader{ reader: r, xorcism: self }
    }

    pub fn writer<W: Write>(self, w: W) -> impl Write {
        XorcismWriter{ writer: w, xorcism: self }
    }

}
