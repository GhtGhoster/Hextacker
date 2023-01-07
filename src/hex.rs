
pub const SQRT_THREE: f32 = 1.7320508;

pub fn flat_hex_to_pixel((q, r): (isize, isize), size: f32) -> (f32, f32) {
    let x = size * 1.5 * (q as f32);
    let y = size * ((SQRT_THREE/2f32) * (q as f32) + SQRT_THREE * (r as f32));
    (x, y)
}

pub fn pixel_to_flat_hex((x, y): (f32, f32), size: f32) -> (isize, isize) {
    let q = (2f32/3f32 * x) / size;
    let r = (-1f32/3f32 * x + SQRT_THREE/3f32 * y) / size;
    let (q, r, _s) = cube_round(axial_to_cube((q, r)));
    (q, r)
}

pub fn axial_to_cube((q, r): (f32, f32)) -> (f32, f32, f32) {
    (q, r, -q-r)
}

pub fn cube_round((fq, fr, fs): (f32, f32, f32)) -> (isize, isize, isize) {
    let mut q = fq.round();
    let mut r = fr.round();
    let mut s = fs.round();

    let q_diff = (q - fq).abs();
    let r_diff = (r - fr).abs();
    let s_diff = (s - fs).abs();

    if q_diff > r_diff && q_diff > s_diff {
        q = -r-s;
    } else if r_diff > s_diff {
        r = -q-s;
    } else {
        s = -q-r;
    }
    return (q as isize, r as isize, s as isize);
}