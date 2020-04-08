use std::io::prelude::*;
use std::fs::File;
use std::num::Wrapping;

const _A_: usize = 0;
const _B_: usize = 1;
const _C_: usize = 2;
const _D_: usize = 3;
const _X_: usize = 0;
const _Y_: usize = 1;
const _Z_: usize = 2;
const _W_: usize = 3;

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
    47, 59, 25, 42, 15, 23, 28, 39, 26, 38, 36, 19, 60, 24, 29, 56,
    37, 63, 20, 61, 55, 2, 30, 44, 9, 10, 6, 22, 53, 48, 51, 11,
    62, 52, 35, 18, 14, 46, 0, 54, 17, 40, 27, 4, 31, 8, 5, 12,
    3, 16, 41, 34, 33, 7, 45, 49, 50, 58, 1, 21, 43, 57, 32, 13
];

static S5: [u32; 32] = [
    20, 26, 7, 31, 19, 12, 10, 15, 22, 30, 13, 14, 4, 24, 9, 18,
    27, 11, 1, 21, 6, 16, 2, 28, 23, 5, 8, 3, 0, 17, 29, 25
];

static S4: [u32; 16] = [
    2, 5, 10, 12, 7, 15, 1, 11, 13, 6, 0, 9, 4, 8, 3, 14
];

static S4_1: [u32; 16] = [
    10, 6, 0, 14, 12, 1, 9, 4, 13, 11, 2, 7, 3, 8, 15, 5
];

static INDEX: [[usize; 4]; 9] = [
    [0, 0, 0, 0],
    [1, 1, 1, 1],
    [2, 2, 2, 2],
    [0, 1, 0, 1],
    [1, 2, 1, 2],
    [2, 0, 2, 0],
    [0, 2, 0, 2],
    [1, 0, 1, 0],
    [2, 1, 2, 1]
];

static ORDER: [[usize; 4]; 12] = [
    [_A_, _B_, _C_, _D_],
    [_B_, _A_, _D_, _C_],
    [_C_, _D_, _A_, _B_],
    [_D_, _C_, _B_, _A_],
    [_A_, _C_, _D_, _B_],
    [_B_, _D_, _C_, _A_],
    [_C_, _A_, _B_, _D_],
    [_D_, _B_, _A_, _C_],
    [_A_, _D_, _B_, _C_],
    [_B_, _C_, _A_, _D_],
    [_C_, _B_, _D_, _A_],
    [_D_, _A_, _C_, _B_]
];

fn rol1(x: u32) -> u32 {
    x.rotate_left(1)
}

fn lf(a: u32, b: u32, mask: u32) -> (u32, u32) {
    (b ^ (a & mask), a ^ (b & !mask))
}

fn mf(a: u32) -> u32 {
    let mut a = a;
    let mut b: u32 = 0;
    for i in 31..=0 {
        if a & 1 != 0 {
            b = b ^ M[i as usize];
        }
        a >>= 1;
    }
    b
}

fn sf(a: u32) -> u32 {
    let i1 = (a >> 26) & 0x3F;
    let i2 = (a >> 21) & 0x1F;
    let i3 = (a >> 16) & 0x1F;
    let i4 = (a >> 11) & 0x1F;
    let i5 = (a >> 6) & 0x1F;
    let i6 = a & 0x3F;
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
    let (e, f) = (a ^ s, b ^ t);
    let (g, h) = (c, d);
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
    let mut m = 1;
    for i in 0..32 {
        let mut x = 0u32;
        if a & m != 0 { x |= 8; }
        if b & m != 0 { x |= 4; }
        if c & m != 0 { x |= 2; }
        if d & m != 0 { x |= 1; }
        x = match r {
            false => S4[x as usize],
            true => S4_1[x as usize]
        };
        if x & 8 != 0 { e |= m; }
        if x & 4 != 0 { f |= m; }
        if x & 2 != 0 { g |= m; }
        if x & 1 != 0 { h |= m; }
        m <<= 1;
    }
    (e, f, g, h)
}

fn iif(a: u32, b: u32, c: u32, d: u32,
       ka: u32, kb: u32, kc: u32, kd: u32) -> (u32, u32, u32, u32) {
    (a ^ ka, b ^ kb, c ^ kc, d ^ kd)
}

fn make_one_imkey(k1: u32, k2: u32, i: u32, j: u32) -> u32 {
    let mut ka = mf(sf(k1));
    let mut kb = mf(sf(k2));
    let mut m = mf(sf(4 * i + j));
    ka = (Wrapping(ka) + Wrapping(m)).0;
    ka &= 0xffffffff;
    kb = (Wrapping(kb) * Wrapping(i + 1)).0;
    kb &= 0xffffffff;
    ka ^= kb;
    return mf(sf(ka));
}

fn make_imkeys(ukey: u128) -> [[u32; 3]; 4] {
    let k1 = ((ukey >> 96) & 0xffffffff) as u32;
    let k2 = ((ukey >> 64) & 0xffffffff) as u32;
    let k3 = ((ukey >> 32) & 0xffffffff) as u32;
    let k4 = (ukey & 0xffffffff) as u32;
    let k5 = k1;
    let k6 = k2;
    let k7 = k3;
    let k8 = k4;

    let mut imkey = [[0u32; 3]; 4];
    for i in 0..3 {
        imkey[_A_][i as usize] = make_one_imkey(k1, k2, i, 0);
        imkey[_B_][i as usize] = make_one_imkey(k3, k4, i, 1);
        imkey[_C_][i as usize] = make_one_imkey(k5, k6, i, 2);
        imkey[_D_][i as usize] = make_one_imkey(k7, k8, i, 3);
    }
    imkey
}

fn make_one_ekey(imkey: [[u32; 3]; 4], t: u32, s: u32) -> u32 {
    let t = t as usize;
    let s = s as usize;
    let mut x = imkey[ORDER[t][_X_]][INDEX[s][_X_]];
    let mut y = imkey[ORDER[t][_Y_]][INDEX[s][_Y_]];
    let mut z = imkey[ORDER[t][_Z_]][INDEX[s][_Z_]];
    let mut w = imkey[ORDER[t][_W_]][INDEX[s][_W_]];
    x = rol1(x);
    x = (Wrapping(x) + Wrapping(y)).0;
    x &= 0xffffffff;
    z = rol1(z);
    z = (Wrapping(z) - Wrapping(w)).0;
    z &= 0xffffffff;
    z = rol1(z);
    x ^= z;
    return x;
}

fn make_ekeys(imkey: [[u32; 3]; 4], num_ekey: u32, ekey: &mut [u32]) {
    for n in 0..num_ekey {
        let t = (n + (n / 36)) % 12;
        let s = n % 9;
        ekey[n as usize] = make_one_ekey(imkey, t, s);
    }
}

fn crypt_block(a: u32, b: u32, c: u32, d: u32, ek: &mut [u32]) -> (u32, u32, u32, u32) {
    let (a, b, c, d) = iif(a, b, c, d, ek[0], ek[1], ek[2], ek[3]);
    let (a, b, c, d) = bf(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[4], ek[5], ek[6], ek[7]);
    let (a, b, c, d) = rf(a, b, c, d, 0x55555555);
    let (c, d, a, b) = rf(a, b, c, d, 0x55555555);

    let (a, b, c, d) = iif(a, b, c, d, ek[8], ek[9], ek[10], ek[11]);
    let (a, b, c, d) = bf(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[12], ek[13], ek[14], ek[15]);
    let (a, b, c, d) = rf(a, b, c, d, 0x33333333);
    let (c, d, a, b) = rf(a, b, c, d, 0x33333333);

    let (a, b, c, d) = iif(a, b, c, d, ek[16], ek[17], ek[18], ek[19]);
    let (a, b, c, d) = bf(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[20], ek[21], ek[22], ek[23]);
    let (a, b, c, d) = rf(a, b, c, d, 0x55555555);
    let (c, d, a, b) = rf(a, b, c, d, 0x55555555);

    let (a, b, c, d) = iif(a, b, c, d, ek[24], ek[25], ek[26], ek[27]);
    let (a, b, c, d) = bf(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[28], ek[29], ek[30], ek[31]);
    let (a, b, c, d) = rf(a, b, c, d, 0x33333333);
    let (c, d, a, b) = rf(a, b, c, d, 0x33333333);

    let (a, b, c, d) = iif(a, b, c, d, ek[32], ek[33], ek[34], ek[35]);
    let (a, b, c, d) = bf(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[36], ek[37], ek[38], ek[39]);
    let (a, b, c, d) = rf(a, b, c, d, 0x55555555);
    let (c, d, a, b) = rf(a, b, c, d, 0x55555555);

    let (a, b, c, d) = iif(a, b, c, d, ek[40], ek[41], ek[42], ek[43]);
    let (a, b, c, d) = bf(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[44], ek[45], ek[46], ek[47]);
    let (a, b, c, d) = rf(a, b, c, d, 0x33333333);
    let (c, d, a, b) = rf(a, b, c, d, 0x33333333);

    let (a, b, c, d) = iif(a, b, c, d, ek[48], ek[49], ek[50], ek[51]);
    let (a, b, c, d) = bf(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[52], ek[53], ek[54], ek[55]);

    (a, b, c, d)
}

fn decrypt_block(a: u32, b: u32, c: u32, d: u32, ek: &mut [u32]) -> (u32, u32, u32, u32) {
    let (a, b, c, d) = iif(a, b, c, d, ek[52], ek[53], ek[54], ek[55]);
    let (a, b, c, d) = bf_1(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[48], ek[49], ek[50], ek[51]);
    let (a, b, c, d) = rf(a, b, c, d, 0x33333333);
    let (c, d, a, b) = rf(a, b, c, d, 0x33333333);

    let (a, b, c, d) = iif(a, b, c, d, ek[44], ek[45], ek[46], ek[47]);
    let (a, b, c, d) = bf_1(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[40], ek[41], ek[42], ek[43]);
    let (a, b, c, d) = rf(a, b, c, d, 0x55555555);
    let (c, d, a, b) = rf(a, b, c, d, 0x55555555);

    let (a, b, c, d) = iif(a, b, c, d, ek[36], ek[37], ek[38], ek[39]);
    let (a, b, c, d) = bf_1(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[32], ek[33], ek[34], ek[35]);
    let (a, b, c, d) = rf(a, b, c, d, 0x33333333);
    let (c, d, a, b) = rf(a, b, c, d, 0x33333333);

    let (a, b, c, d) = iif(a, b, c, d, ek[28], ek[29], ek[30], ek[31]);
    let (a, b, c, d) = bf_1(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[24], ek[25], ek[26], ek[27]);
    let (a, b, c, d) = rf(a, b, c, d, 0x55555555);
    let (c, d, a, b) = rf(a, b, c, d, 0x55555555);

    let (a, b, c, d) = iif(a, b, c, d, ek[20], ek[21], ek[22], ek[23]);
    let (a, b, c, d) = bf_1(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[16], ek[17], ek[18], ek[19]);
    let (a, b, c, d) = rf(a, b, c, d, 0x33333333);
    let (c, d, a, b) = rf(a, b, c, d, 0x33333333);

    let (a, b, c, d) = iif(a, b, c, d, ek[12], ek[13], ek[14], ek[15]);
    let (a, b, c, d) = bf_1(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[8], ek[9], ek[10], ek[11]);
    let (a, b, c, d) = rf(a, b, c, d, 0x55555555);
    let (c, d, a, b) = rf(a, b, c, d, 0x55555555);

    let (a, b, c, d) = iif(a, b, c, d, ek[4], ek[5], ek[6], ek[7]);
    let (a, b, c, d) = bf_1(a, b, c, d);
    let (a, b, c, d) = iif(a, b, c, d, ek[0], ek[1], ek[2], ek[3]);

    (a, b, c, d)
}

pub fn crypt(name: &str, key: u128) {
    let imkeys = make_imkeys(key);
    let mut ek = [0u32; 56];
    make_ekeys(imkeys, 56, &mut ek);
    let mut f = File::open(name).unwrap();
    let mut fo = File::create(format!("./crypted-{}", name)).unwrap();
    let mut buffer = [0; 16];
    loop {
        let n = f.read(&mut buffer[..]).unwrap();
        match n {
            0 => break,
            1..=15 => (),
            _ => {
                let a = ((buffer[0] as u32) << 24) | ((buffer[1] as u32) << 16) | ((buffer[2] as u32) << 8) | buffer[3] as u32;
                let b = ((buffer[4] as u32) << 24) | ((buffer[5] as u32) << 16) | ((buffer[6] as u32) << 8) | buffer[7] as u32;
                let c = ((buffer[8] as u32) << 24) | ((buffer[9] as u32) << 16) | ((buffer[10] as u32) << 8) | buffer[11] as u32;
                let d = ((buffer[12] as u32) << 24) | ((buffer[13] as u32) << 16) | ((buffer[14] as u32) << 8) | buffer[15] as u32;
                let (a, b, c, d) = crypt_block(a, b, c, d, &mut ek);
                let mut out_buffer = [0; 16];
                out_buffer[0] = (a >> 24) as u8;
                out_buffer[1] = (a >> 16) as u8;
                out_buffer[2] = (a >> 8) as u8;
                out_buffer[3] = a as u8;
                out_buffer[4] = (b >> 24) as u8;
                out_buffer[5] = (b >> 16) as u8;
                out_buffer[6] = (b >> 8) as u8;
                out_buffer[7] = b as u8;
                out_buffer[8] = (c >> 24) as u8;
                out_buffer[9] = (c >> 16) as u8;
                out_buffer[10] = (c >> 8) as u8;
                out_buffer[11] = c as u8;
                out_buffer[12] = (d >> 24) as u8;
                out_buffer[13] = (d >> 16) as u8;
                out_buffer[14] = (d >> 8) as u8;
                out_buffer[15] = d as u8;
                fo.write(&out_buffer);
            }
        }
    }
}

pub fn decrypt(name: &str, key: u128) {
    let imkeys = make_imkeys(key);
    let mut ek = [0u32; 56];
    make_ekeys(imkeys, 56, &mut ek);
    let mut f = File::open(name).unwrap();
    let mut fo = File::create(format!("./decrypted-{}", name)).unwrap();
    let mut buffer = [0; 16];
    loop {
        let n = f.read(&mut buffer[..]).unwrap();
        match n {
            0 => break,
            1..=15 => (),
            _ => {
                let a = ((buffer[0] as u32) << 24) | ((buffer[1] as u32) << 16) | ((buffer[2] as u32) << 8) | buffer[3] as u32;
                let b = ((buffer[4] as u32) << 24) | ((buffer[5] as u32) << 16) | ((buffer[6] as u32) << 8) | buffer[7] as u32;
                let c = ((buffer[8] as u32) << 24) | ((buffer[9] as u32) << 16) | ((buffer[10] as u32) << 8) | buffer[11] as u32;
                let d = ((buffer[12] as u32) << 24) | ((buffer[13] as u32) << 16) | ((buffer[14] as u32) << 8) | buffer[15] as u32;
                let (a, b, c, d) = decrypt_block(a, b, c, d, &mut ek);
                let mut out_buffer = [0; 16];
                out_buffer[0] = (a >> 24) as u8;
                out_buffer[1] = (a >> 16) as u8;
                out_buffer[2] = (a >> 8) as u8;
                out_buffer[3] = a as u8;
                out_buffer[4] = (b >> 24) as u8;
                out_buffer[5] = (b >> 16) as u8;
                out_buffer[6] = (b >> 8) as u8;
                out_buffer[7] = b as u8;
                out_buffer[8] = (c >> 24) as u8;
                out_buffer[9] = (c >> 16) as u8;
                out_buffer[10] = (c >> 8) as u8;
                out_buffer[11] = c as u8;
                out_buffer[12] = (d >> 24) as u8;
                out_buffer[13] = (d >> 16) as u8;
                out_buffer[14] = (d >> 8) as u8;
                out_buffer[15] = d as u8;
                fo.write(&out_buffer);
            }
        }
    }
}
