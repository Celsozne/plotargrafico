use plotters::prelude::*;

pub fn plot_bifurcation(
    points: &[(f64, f64)],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (1000, 700)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Diagrama de Bifurcação do Mapa Logístico",
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(1.0f64..4.0f64, 0.0f64..1.0f64)?;

    chart.configure_mesh().x_desc("r").y_desc("x").draw()?;

    chart.draw_series(
        points
            .iter()
            .map(|(r, x)| Circle::new((*r, *x), 1, RED.filled())),
    )?;

    root.present()?;
    Ok(())
}
