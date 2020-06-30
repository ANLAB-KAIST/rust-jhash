use std;

#[inline(always)]
fn rotate_left_u32(x: u32, k: usize) -> u32 {
    (((x) << (k)) | ((x) >> (32 - (k))))
}

#[inline(always)]
pub fn jhash_mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = a.wrapping_sub(*c);
    *a ^= rotate_left_u32(*c, 4);
    *c = c.wrapping_add(*b);

    *b = b.wrapping_sub(*a);
    *b ^= rotate_left_u32(*a, 6);
    *a = a.wrapping_add(*c);

    *c = c.wrapping_sub(*b);
    *c ^= rotate_left_u32(*b, 8);
    *b = b.wrapping_add(*a);

    *a = a.wrapping_sub(*c);
    *a ^= rotate_left_u32(*c, 16);
    *c = c.wrapping_add(*b);

    *b = b.wrapping_sub(*a);
    *b ^= rotate_left_u32(*a, 19);
    *a = a.wrapping_add(*c);

    *c = c.wrapping_sub(*b);
    *c ^= rotate_left_u32(*b, 4);
    *b = b.wrapping_add(*a);
}

#[inline(always)]
pub fn jhash_final(mut a: u32, mut b: u32, mut c: u32) -> u32 {
    c ^= b;
    c = c.wrapping_sub(rotate_left_u32(b, 14));

    a ^= c;
    a = a.wrapping_sub(rotate_left_u32(c, 11));

    b ^= a;
    b = b.wrapping_sub(rotate_left_u32(a, 25));

    c ^= b;
    c = c.wrapping_sub(rotate_left_u32(b, 16));

    a ^= c;
    a = a.wrapping_sub(rotate_left_u32(c, 4));

    b ^= a;
    b = b.wrapping_sub(rotate_left_u32(a, 14));

    c ^= b;
    c = c.wrapping_sub(rotate_left_u32(b, 24));
    c
}

pub const JHASH_INITVAL: u32 = 0xdeadbeef;

#[inline(always)]
pub fn jhash(key: &[u8], initval: u32) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;

    let total_length = key.len();
    let mut length = 0usize;

    a = JHASH_INITVAL
        .wrapping_add(length as u32)
        .wrapping_add(initval);
    b = a;
    c = a;

    let mut k: *const u8 = key.as_ptr();
    while length + 12 <= total_length {
        a = a.wrapping_add(unsafe { *(k as *const u32) });
        k = unsafe { k.offset(4) };
        b = b.wrapping_add(unsafe { *(k as *const u32) });
        k = unsafe { k.offset(4) };
        c = c.wrapping_add(unsafe { *(k as *const u32) });
        k = unsafe { k.offset(4) };
        jhash_mix(&mut a, &mut b, &mut c);
        length += 12;
    }
    let final_bytes = &key[length..];
    match final_bytes.len() {
        12 => {
            c = c.wrapping_add((final_bytes[11] as u32) << 24);
            c = c.wrapping_add((final_bytes[10] as u32) << 16);
            c = c.wrapping_add((final_bytes[9] as u32) << 8);
            c = c.wrapping_add(final_bytes[8] as u32);
            b = b.wrapping_add((final_bytes[7] as u32) << 24);
            b = b.wrapping_add((final_bytes[6] as u32) << 16);
            b = b.wrapping_add((final_bytes[5] as u32) << 8);
            b = b.wrapping_add(final_bytes[4] as u32);
            a = a.wrapping_add((final_bytes[3] as u32) << 24);
            a = a.wrapping_add((final_bytes[2] as u32) << 16);
            a = a.wrapping_add((final_bytes[1] as u32) << 8);
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        11 => {
            c = c.wrapping_add((final_bytes[10] as u32) << 16);
            c = c.wrapping_add((final_bytes[9] as u32) << 8);
            c = c.wrapping_add(final_bytes[8] as u32);
            b = b.wrapping_add((final_bytes[7] as u32) << 24);
            b = b.wrapping_add((final_bytes[6] as u32) << 16);
            b = b.wrapping_add((final_bytes[5] as u32) << 8);
            b = b.wrapping_add(final_bytes[4] as u32);
            a = a.wrapping_add((final_bytes[3] as u32) << 24);
            a = a.wrapping_add((final_bytes[2] as u32) << 16);
            a = a.wrapping_add((final_bytes[1] as u32) << 8);
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        10 => {
            c = c.wrapping_add((final_bytes[9] as u32) << 8);
            c = c.wrapping_add(final_bytes[8] as u32);
            b = b.wrapping_add((final_bytes[7] as u32) << 24);
            b = b.wrapping_add((final_bytes[6] as u32) << 16);
            b = b.wrapping_add((final_bytes[5] as u32) << 8);
            b = b.wrapping_add(final_bytes[4] as u32);
            a = a.wrapping_add((final_bytes[3] as u32) << 24);
            a = a.wrapping_add((final_bytes[2] as u32) << 16);
            a = a.wrapping_add((final_bytes[1] as u32) << 8);
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        9 => {
            c = c.wrapping_add(final_bytes[8] as u32);
            b = b.wrapping_add((final_bytes[7] as u32) << 24);
            b = b.wrapping_add((final_bytes[6] as u32) << 16);
            b = b.wrapping_add((final_bytes[5] as u32) << 8);
            b = b.wrapping_add(final_bytes[4] as u32);
            a = a.wrapping_add((final_bytes[3] as u32) << 24);
            a = a.wrapping_add((final_bytes[2] as u32) << 16);
            a = a.wrapping_add((final_bytes[1] as u32) << 8);
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        8 => {
            b = b.wrapping_add((final_bytes[7] as u32) << 24);
            b = b.wrapping_add((final_bytes[6] as u32) << 16);
            b = b.wrapping_add((final_bytes[5] as u32) << 8);
            b = b.wrapping_add(final_bytes[4] as u32);
            a = a.wrapping_add((final_bytes[3] as u32) << 24);
            a = a.wrapping_add((final_bytes[2] as u32) << 16);
            a = a.wrapping_add((final_bytes[1] as u32) << 8);
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        7 => {
            b = b.wrapping_add((final_bytes[6] as u32) << 16);
            b = b.wrapping_add((final_bytes[5] as u32) << 8);
            b = b.wrapping_add(final_bytes[4] as u32);
            a = a.wrapping_add((final_bytes[3] as u32) << 24);
            a = a.wrapping_add((final_bytes[2] as u32) << 16);
            a = a.wrapping_add((final_bytes[1] as u32) << 8);
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        6 => {
            b = b.wrapping_add((final_bytes[5] as u32) << 8);
            b = b.wrapping_add(final_bytes[4] as u32);
            a = a.wrapping_add((final_bytes[3] as u32) << 24);
            a = a.wrapping_add((final_bytes[2] as u32) << 16);
            a = a.wrapping_add((final_bytes[1] as u32) << 8);
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        5 => {
            b = b.wrapping_add(final_bytes[4] as u32);
            a = a.wrapping_add((final_bytes[3] as u32) << 24);
            a = a.wrapping_add((final_bytes[2] as u32) << 16);
            a = a.wrapping_add((final_bytes[1] as u32) << 8);
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        4 => {
            a = a.wrapping_add((final_bytes[3] as u32) << 24);
            a = a.wrapping_add((final_bytes[2] as u32) << 16);
            a = a.wrapping_add((final_bytes[1] as u32) << 8);
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        3 => {
            a = a.wrapping_add((final_bytes[2] as u32) << 16);
            a = a.wrapping_add((final_bytes[1] as u32) << 8);
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        2 => {
            a = a.wrapping_add((final_bytes[1] as u32) << 8);
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        1 => {
            a = a.wrapping_add(final_bytes[0] as u32);
        }
        0 => {}
        _ => {
            panic!("Never happen");
        }
    }
    return jhash_final(a, b, c);
}

#[inline(always)]
pub fn jhash2(key: &[u32], initval: u32) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;

    let total_length = key.len();
    let mut length = 0usize;

    a = JHASH_INITVAL
        .wrapping_add(length as u32)
        .wrapping_add(initval);
    b = a;
    c = a;

    /* Handle most of the key */
    while length <= total_length - 3 {
        a = a.wrapping_add(key[length + 0]);
        b = b.wrapping_add(key[length + 1]);
        c = c.wrapping_add(key[length + 2]);
        jhash_mix(&mut a, &mut b, &mut c);
        length += 3;
    }

    let final_bytes = &key[length..];
    match final_bytes.len() {
        3 => {
            c = c.wrapping_add(final_bytes[2]);
            b = b.wrapping_add(final_bytes[1]);
            a = a.wrapping_add(final_bytes[0]);
        }
        2 => {
            b = b.wrapping_add(final_bytes[1]);
            a = a.wrapping_add(final_bytes[0]);
        }
        1 => {
            a = a.wrapping_add(final_bytes[0]);
        }
        0 => {}
        _ => {
            panic!("Never happen");
        }
    }
    return jhash_final(a, b, c);
}

#[inline(always)]
fn jhash_nwords(mut a: u32, mut b: u32, mut c: u32, initval: u32) -> u32 {
    a = a.wrapping_add(initval);
    b = b.wrapping_add(initval);
    c = c.wrapping_add(initval);

    return jhash_final(a, b, c);
}

#[inline(always)]
pub fn jhash_3words(a: u32, b: u32, c: u32, initval: u32) -> u32 {
    return jhash_nwords(
        a,
        b,
        c,
        initval.wrapping_add(JHASH_INITVAL).wrapping_add(3 << 2),
    );
}

#[inline(always)]
pub fn jhash_2words(a: u32, b: u32, initval: u32) -> u32 {
    return jhash_nwords(
        a,
        b,
        0,
        initval.wrapping_add(JHASH_INITVAL).wrapping_add(2 << 2),
    );
}

#[inline(always)]
pub fn jhash_1words(a: u32, initval: u32) -> u32 {
    return jhash_nwords(
        a,
        0,
        0,
        initval.wrapping_add(JHASH_INITVAL).wrapping_add(1 << 2),
    );
}

enum JHashBuffer {
    None,
    One(u32),
    Two(u32, u32),
}

impl Default for JHashBuffer {
    #[inline(always)]
    fn default() -> JHashBuffer {
        JHashBuffer::None
    }
}

#[derive(Default)]
pub struct JHasher {
    current: u32,
    buffer: JHashBuffer,
}

impl JHasher {
    #[inline(always)]
    pub fn new(initval: u32) -> JHasher {
        JHasher {
            current: initval,
            buffer: JHashBuffer::None,
        }
    }

    #[inline(always)]
    fn flush_buffer(&mut self) {
        match self.buffer {
            JHashBuffer::None => {}
            JHashBuffer::One(val1) => {
                self.current = jhash_1words(val1, self.current);
                self.buffer = JHashBuffer::None;
            }
            JHashBuffer::Two(val1, val2) => {
                self.current = jhash_2words(val1, val2, self.current);
                self.buffer = JHashBuffer::None;
            }
        }
    }
}

#[inline(always)]
#[cfg(target_endian = "little")]
fn split_u64(val: u64) -> (u32, u32) {
    (((val >> 32) as u32), ((val & 0x00000000FFFFFFFFu64) as u32))
}

#[cfg(target_endian = "big")]
fn split_u64(val: u64) -> (u32, u32) {
    (((val & 0x00000000FFFFFFFFu64) as u32), ((val >> 32) as u32))
}

impl std::hash::Hasher for JHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        match self.buffer {
            JHashBuffer::None => self.current as u64,
            JHashBuffer::One(val1) => jhash_1words(val1, self.current) as u64,
            JHashBuffer::Two(val1, val2) => jhash_2words(val1, val2, self.current) as u64,
        }
    }
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.flush_buffer();
        self.current = jhash(bytes, self.current);
    }
    #[inline(always)]
    fn write_u32(&mut self, val: u32) {
        match self.buffer {
            JHashBuffer::None => {
                self.buffer = JHashBuffer::One(val);
            }
            JHashBuffer::One(val1) => {
                self.buffer = JHashBuffer::Two(val1, val);
            }
            JHashBuffer::Two(val1, val2) => {
                self.current = jhash_3words(val1, val2, val, self.current);
                self.buffer = JHashBuffer::None;
            }
        }
    }
    #[inline(always)]
    fn write_u64(&mut self, val: u64) {
        let (val_a, val_b) = split_u64(val);

        match self.buffer {
            JHashBuffer::None => {
                self.buffer = JHashBuffer::Two(val_a, val_b);
            }
            JHashBuffer::One(val1) => {
                self.current = jhash_3words(val1, val_a, val_b, self.current);
                self.buffer = JHashBuffer::None;
            }
            JHashBuffer::Two(val1, val2) => {
                self.current = jhash_3words(val1, val2, val_a, self.current);
                self.buffer = JHashBuffer::One(val_b);
            }
        }
    }
    #[inline(always)]
    fn write_i32(&mut self, val: i32) {
        self.write_u32(val as u32)
    }
    #[inline(always)]
    fn write_i64(&mut self, val: i64) {
        self.write_u64(val as u64)
    }
}

#[derive(Default, Clone, Debug)]
pub struct JHashBuilder {
    initial_value: u32,
}

impl JHashBuilder {
    pub fn new(initial_value: u32) -> JHashBuilder {
        JHashBuilder {
            initial_value: initial_value,
        }
    }
}

impl std::hash::BuildHasher for JHashBuilder {
    type Hasher = JHasher;
    fn build_hasher(&self) -> Self::Hasher {
        JHasher::new(self.initial_value)
    }
}
