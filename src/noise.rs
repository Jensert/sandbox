pub fn hash(x: i32, y: i32, seed: i32) -> f32 {
    let mut h = x
        .wrapping_mul(374761393)
        .wrapping_add(y.wrapping_mul(668265263))
        .wrapping_add(seed.wrapping_mul(1442695041));

    h ^= h >> 13;
    h = h.wrapping_mul(1_274_126_177);
    h ^= h >> 16;

    (h as u32) as f32 / u32::MAX as f32
}

pub fn smoothstep(t: f32) -> f32 {
    return t * t * (3.0 - 2.0 * t);
}

pub fn lerp(start: f32, stop: f32, step: f32) -> f32 {
    return start + step * (stop - start);
}

pub fn noise2d(x: f32, y: f32, seed: i32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;

    let sx = smoothstep(x - x0 as f32);
    let sy = smoothstep(y - y0 as f32);

    let n00 = hash(x0, y0, seed);
    let n10 = hash(x1, y0, seed);
    let n01 = hash(x0, y1, seed);
    let n11 = hash(x1, y1, seed);

    let ix0 = lerp(n00, n10, sx);
    let ix1 = lerp(n01, n11, sx);

    return lerp(ix0, ix1, sy);
}

pub fn surface_height(world_x: i32, world_sea_level: f32, seed: i32) -> i32 {
    let amplitude = 80.0; // increase this for higher hills
    let n = noise2d(
        world_x as f32 * 0.02, // increase multiplication to make terrain more jagged
        0.0,
        seed,
    );
    (world_sea_level + n * amplitude) as i32
}
