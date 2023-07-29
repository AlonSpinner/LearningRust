use plotters::prelude::*;

pub fn plot_theta_vecs(time_values : Vec<f64>, theta_values: Vec<Vec<f64>>, titles : Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    //make sure that the length of element in theta_values is the same as length of time_values
        for i in 0..theta_values.len(){
            assert_eq!(theta_values[i].len(), time_values.len());
        }
    let root = SVGBackend::new("plot.svg", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Pendulum Motion", ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(time_values[0]..time_values[time_values.len() -1], -3f64..3f64)?;

    chart.configure_mesh()
        .x_desc("Time (s)")
        .y_desc("Angle (rad)")
        .draw()?;

    let n = theta_values.len();
    let c0_t = (255.0, 0.0, 0.0);
    let cn_t = (0.0, 0.0, 255.0);
    let colors: Vec<RGBColor> = (0..n).map(|i| {
        let ratio = i as f64 / n as f64;
        let r = c0_t.0 + (cn_t.0 - c0_t.0) * ratio;
        let g = c0_t.1 + (cn_t.1 - c0_t.1) * ratio;
        let b = c0_t.2 + (cn_t.2 - c0_t.2) * ratio;
        RGBColor(r as u8, g as u8, b as u8)
    }).collect();

    for i in 0..n{
        let color = colors[i].clone();
        chart.draw_series(LineSeries::new(
            time_values.iter().zip(theta_values[i].iter()).map(|(x, y)| (*x, *y)),
            color,
        ))?.label(titles[i])
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
    }
    
    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    root.present()?;

    Ok(())
}
