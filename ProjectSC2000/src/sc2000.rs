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

fn generate_ek(key: u128) {
    let mut uk: [u32; 8] = [0; 8];
    uk[3] = (key & 0xffffffff) as u32;
    uk[2] = ((key >> 8) & 0xffffffff) as u32;
    uk[1] = ((key >> 16) & 0xffffffff) as u32;
    uk[0] = ((key >> 24) & 0xffffffff) as u32;
    uk[7] = uk[3];
    uk[6] = uk[2];
    uk[5] = uk[1];
    uk[4] = uk[0];
    let mut aa: [u32; 3] = [0; 3];
    let mut bb: [u32; 3] = [0; 3];
    let mut cc: [u32; 3] = [0; 3];
    let mut dd: [u32; 3] = [0; 3];
    for i in 0..3 {
        aa[i as usize] = wf(4 * i, uk[0], uk[1], i + 1);
        bb[i as usize] = wf(4 * i + 1, uk[2], uk[3], i + 1);
        cc[i as usize] = wf(4 * i + 2, uk[4], uk[5], i + 1);
        dd[i as usize] = wf(4 * i + 3, uk[6], uk[7], i + 1);
    }
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
}

static mut EK: [u32; 56] = [0; 56];

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

static INDEX: [&[u32]; 4] = [
    &[0, 1, 2, 0, 1, 2, 0, 1, 2],
    &[0, 1, 2, 1, 2, 0, 2, 0, 1],
    &[0, 1, 2, 0, 1, 2, 0, 1, 2],
    &[0, 1, 2, 1, 2, 0, 2, 0, 1]
];
