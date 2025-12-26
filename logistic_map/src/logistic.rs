pub fn bifurcation_points(
    r_min: f64,
    r_max: f64,
    r_steps: usize,
    x0: f64,
    transient: usize,
    keep: usize,
) -> Vec<(f64, f64)> {
    let mut points = Vec::new();

    for i in 0..r_steps {
        let r = r_min + (r_max - r_min) * (i as f64) / (r_steps as f64);

        let mut x = x0;
        for _ in 0..transient {
            x = r * x * (1.0 - x);
        }

        for _ in 0..keep {
            x = r * x * (1.0 - x);
            points.push((r, x));
        }
    }

    points
}
