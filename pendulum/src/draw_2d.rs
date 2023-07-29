use plotters::prelude::*;

pub fn plot(theta_values: Vec<f64>, d_theta_values: Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Pendulum Motion", ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..10f64, -2f64..2f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        theta_values.into_iter().enumerate().map(|(i, theta)| (i as f64, theta)),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        d_theta_values.into_iter().enumerate().map(|(i, d_theta)| (i as f64, d_theta)),
        &BLUE,
    ))?;

    root.present()?;

    Ok(())
}
