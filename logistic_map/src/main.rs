use logistic_map::logistic::bifurcation_points;
use logistic_map::plot::plot_bifurcation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r_min = 1.0;
    let r_max = 10.0;
    let r_steps = 1000;

    let x0 = 0.7;
    let transient = 2000;
    let keep = 200;

    let points = bifurcation_points(r_min, r_max, r_steps, x0, transient, keep);
    plot_bifurcation(&points, "bifurcation.png")?;

    println!("Imagem gerada: bifurcation.png");
    Ok(())
}
