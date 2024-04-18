pub fn abs(x: f32) -> f32 {
    if x < 0.0f32 {
        return -x;
    }

    return x;
}

pub fn min(x: f32, y: f32) -> f32 {
    if x < y {
        return x;
    }
    return y;
}

pub fn max(x: f32, y: f32) -> f32 {
    if x < y {
        return y;
    }
    return x;
}
