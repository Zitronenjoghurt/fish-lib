pub fn float_interpolate(a: f32, b: f32, factor: f32) -> f32 {
    a + ((b - a) * factor)
}
