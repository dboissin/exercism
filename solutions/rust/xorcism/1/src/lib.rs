use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    key_length: usize,
    key_idx: usize,
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
            data[i] = data[i] ^ self.get_key_byte();
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

}
