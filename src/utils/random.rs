use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Normal;

pub fn random_normal(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    let mean = (min + max) / 2.0;
    let std_dev = (min - max) / 6.0;

    let normal = Normal::new(mean, std_dev).unwrap();
    let size = normal.sample(&mut rng);

    size.clamp(min, max)
}
