use std::io::prelude::*;
use std::fs::File;

fn lf(a: u32, b: u32, mask: u32) -> (u32, u32) {
    (b ^ (a & mask), a ^ (b & !mask))
}

fn mf(a: u32) -> u32 {
    let mut b: u32 = 0;
    for i in 0..32 {
        if bit_at(a, i) == 1 {
            b = b ^ M[i as usize];
        }
    }
    b
}

fn sf(a: u32) -> u32 {
    let i1 = bits_at(a, 0, 5);
    let i2 = bits_at(a, 6, 10);
    let i3 = bits_at(a, 11, 15);
    let i4 = bits_at(a, 16, 20);
    let i5 = bits_at(a, 21, 25);
    let i6 = bits_at(a, 26, 31);
    let s1 = S6[i1 as usize] << 26;
    let s2 = S5[i2 as usize] << 21;
    let s3 = S5[i3 as usize] << 16;
    let s4 = S5[i4 as usize] << 11;
    let s5 = S5[i5 as usize] << 6;
    let s6 = S6[i6 as usize];
    s1 | s2 | s3 | s4 | s5 | s6
}

fn ff(a: u32, b: u32, mask: u32) -> (u32, u32) {
    lf(mf(sf(a)), mf(sf(b)), mask)
}

fn rf(a: u32, b: u32, c: u32, d: u32, mask: u32) -> (u32, u32, u32, u32) {
    let (s, t) = ff(c, d, mask);
    let (g, h) = (a ^ s, b ^ t);
    let (s, t) = ff(g, h, mask);
    let (e, f) = (c ^ s, d ^ t);
    (e, f, g, h)
}

fn bf(a: u32, b: u32, c: u32, d: u32) -> (u32, u32, u32, u32) {
    bf_helper(a, b, c, d, false)
}

fn bf_1(a: u32, b: u32, c: u32, d: u32) -> (u32, u32, u32, u32) {
    bf_helper(a, b, c, d, true)
}

fn bf_helper(a: u32, b: u32, c: u32, d: u32, r: bool) -> (u32, u32, u32, u32) {
    let mut e = 0;
    let mut f = 0;
    let mut g = 0;
    let mut h = 0;
    for i in 0..32 {
        let mut x: u32 = 0;
        x |= bit_at(a, i) << 3;
        x |= bit_at(b, i) << 2;
        x |= bit_at(c, i) << 1;
        x |= bit_at(d, i);
        x = match r {
            false => S4[x as usize],
            true => S4_1[x as usize]
        };
        e |= (x >> 3) << (31 - i);
        f |= (x >> 2) << (31 - i);
        g |= (x >> 1) << (31 - i);
        h |= x << (31 - i);
    }
    (e, f, g, h)
}

fn gf(a: u32, b: u32, c: u32, d: u32) -> u32 {
    (rol1(a) + b) ^ rol1(rol1(c) - d)
}

fn wf(a: u32, b: u32, c: u32, d: u32) -> u32 {
    mf(sf((mf(sf(a)) + mf(sf(b))) ^ (mf(sf(c)) * d)))
}

fn rol1(x: u32) -> u32 {
    x.rotate_left(1)
}

// x = x_0 || x_1 || x_2 || ... || x_31
fn bit_at(x: u32, i: u32) -> u32 {
    (x >> (31 - i)) & 1
}

// x = x_0 || x_1 || x_2 || ... || x_31
fn bits_at(x: u32, i1: u32, i2: u32) -> u32 {
    (x >> (31 - i2)) & !(!0 << (i2 - i1 + 1))
}

fn generate_ek(key: u128) -> [u32; 56] {
    let mut uk = [0u32; 8];
    uk[3] = (key & 0xffffffff) as u32;
    uk[2] = ((key >> 8) & 0xffffffff) as u32;
    uk[1] = ((key >> 16) & 0xffffffff) as u32;
    uk[0] = ((key >> 24) & 0xffffffff) as u32;
    uk[7] = uk[3];
    uk[6] = uk[2];
    uk[5] = uk[1];
    uk[4] = uk[0];
    let mut aa = [0u32; 3];
    let mut bb = [0u32; 3];
    let mut cc = [0u32; 3];
    let mut dd = [0u32; 3];
    for i in 0..3 {
        aa[i as usize] = wf(4 * i, uk[0], uk[1], i + 1);
        bb[i as usize] = wf(4 * i + 1, uk[2], uk[3], i + 1);
        cc[i as usize] = wf(4 * i + 2, uk[4], uk[5], i + 1);
        dd[i as usize] = wf(4 * i + 3, uk[6], uk[7], i + 1);
    }
    let mut ek = [0u32; 56];
    for n in 0..56 {
        let u = n % 9;
        let v = (n + n / 36) % 12;
        let x = INDEX[0][u];
        let y = INDEX[1][u];
        let z = INDEX[2][u];
        let w = INDEX[3][u];
        match v {
            0 => ek[n] = gf(aa[x], bb[y], cc[z], dd[w]),
            1 => ek[n] = gf(bb[x], aa[y], dd[z], cc[w]),
            2 => ek[n] = gf(cc[x], dd[y], aa[z], bb[w]),
            3 => ek[n] = gf(dd[x], cc[y], bb[z], aa[w]),
            4 => ek[n] = gf(aa[x], cc[y], dd[z], bb[w]),
            5 => ek[n] = gf(bb[x], dd[y], cc[z], aa[w]),
            6 => ek[n] = gf(cc[x], aa[y], bb[z], dd[w]),
            7 => ek[n] = gf(dd[x], bb[y], aa[z], cc[w]),
            8 => ek[n] = gf(aa[x], dd[y], bb[z], cc[w]),
            9 => ek[n] = gf(bb[x], cc[y], aa[z], dd[w]),
            10 => ek[n] = gf(cc[x], bb[y], dd[z], aa[w]),
            11 => ek[n] = gf(dd[x], aa[y], cc[z], bb[w]),
            _ => {}
        }
    }
    ek
}

pub fn encode(name: String, key: u128) {
    let mut ek = generate_ek(key);
    let mut f = File::open(name)?;
    let mut fo = File::create(name + ".en")?;
    let mut buffer = [0; 16];
    loop {
        let n = f.read(&mut buffer[..])?;
        match n {
            0 => break,
            1...15 => (),
            _ => {
                let mut c0 = 0x55555555u32;
                let mut c1 = 0x33333333u32;
                let mut e = ((buffer[0] as u32) << 24) | ((buffer[1] as u32) << 16) | ((buffer[2] as u32) << 8) | buffer[3] as u32;
                let mut f = ((buffer[4] as u32) << 24) | ((buffer[5] as u32) << 16) | ((buffer[6] as u32) << 8) | buffer[7] as u32;
                let mut g = ((buffer[8] as u32) << 24) | ((buffer[9] as u32) << 16) | ((buffer[10] as u32) << 8) | buffer[11] as u32;
                let mut h = ((buffer[12] as u32) << 24) | ((buffer[13] as u32) << 16) | ((buffer[14] as u32) << 8) | buffer[15] as u32;
                for i in 0..6 {
                    (e, f, g, h) = (e ^ ek[8 * i], f ^ ek[8 * i + 1], g ^ ek[8 * i + 2], h ^ ek[8 * i + 3]);
                    (e, f, g, h) = bf(e, f, g, h);
                    (e, f, g, h) = (e ^ ek[8 * i + 4], f ^ ek[8 * i + 5], g ^ ek[8 * i + 6], h ^ ek[8 * i + 7]);
                    (e, f, g, h) = rf(e, f, g, h, c0);
                    (c0, c1) = (c1, c0);
                }
                (e, f, g, h) = (e ^ ek[48], f ^ ek[49], g ^ ek[50], h ^ ek[51]);
                (e, f, g, h) = bf(e, f, g, h);
                (e, f, g, h) = (e ^ ek[52], f ^ ek[53], g ^ ek[54], h ^ ek[55]);
                let mut out_buffer = [0; 16];
                out_buffer[0] = (e >> 24) as u8;
                out_buffer[1] = (e >> 16) as u8;
                out_buffer[2] = (e >> 8) as u8;
                out_buffer[3] = e as u8;
                out_buffer[4] = (f >> 24) as u8;
                out_buffer[5] = (f >> 16) as u8;
                out_buffer[6] = (f >> 8) as u8;
                out_buffer[7] = f as u8;
                out_buffer[8] = (g >> 24) as u8;
                out_buffer[9] = (g >> 16) as u8;
                out_buffer[10] = (g >> 8) as u8;
                out_buffer[11] = g as u8;
                out_buffer[12] = (h >> 24) as u8;
                out_buffer[13] = (h >> 16) as u8;
                out_buffer[14] = (h >> 8) as u8;
                out_buffer[15] = h as u8;
                fo.write(&out_buffer);
            }
        }
    }
}

static M: [u32; 32] = [
    0xd0c19225, 0xa5a2240a, 0x1b84d250, 0xb728a4a1,
    0x6a704902, 0x85dddbe6, 0x766ff4a4, 0xecdfe128,
    0xafd13e94, 0xdf837d09, 0xbb27fa52, 0x695059ad,
    0x52a1bb58, 0xcc322f1d, 0x1844565b, 0xb4a8acf6,
    0x34235438, 0x6847a851, 0xe48c0cbb, 0xcd181136,
    0x9a112a0c, 0x43ec6d0e, 0x87d8d27d, 0x487dc995,
    0x90fb9b4b, 0xa1f63697, 0xfc513ed9, 0x78a37d93,
    0x8d16c5df, 0x9e0c8bbe, 0x3c381f7c, 0xe9fb0779
];

static S6: [u32; 64] = [
    47, 59, 25, 42, 16, 23, 28, 39, 26, 38, 36, 19, 60, 24, 39, 56,
    37, 63, 20, 61, 56, 02, 30, 44, 08, 10, 06, 22, 53, 47, 51, 11,
    62, 52, 35, 18, 14, 46, 00, 54, 17, 40, 27, 04, 31, 08, 05, 12,
    03, 16, 41, 34, 33, 07, 45, 49, 50, 58, 01, 21, 43, 57, 32, 13
];

static S5: [u32; 32] = [
    20, 26, 07, 31, 19, 12, 10, 15, 22, 30, 13, 14, 04, 24, 09, 18,
    27, 11, 01, 21, 06, 16, 02, 28, 23, 05, 08, 03, 00, 17, 29, 25
];

static S4: [u32; 16] = [
    02, 05, 10, 12, 07, 15, 01, 11, 13, 06, 00, 09, 04, 08, 03, 14
];

static S4_1: [u32; 16] = [
    10, 06, 00, 14, 12, 01, 09, 04, 13, 11, 02, 07, 03, 08, 15, 05
];

static INDEX: [&[usize]; 4] = [
    &[0, 1, 2, 0, 1, 2, 0, 1, 2],
    &[0, 1, 2, 1, 2, 0, 2, 0, 1],
    &[0, 1, 2, 0, 1, 2, 0, 1, 2],
    &[0, 1, 2, 1, 2, 0, 2, 0, 1]
];
