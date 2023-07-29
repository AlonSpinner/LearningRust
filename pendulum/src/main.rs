use pendulum::{make_propogate_euler, plot};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let l = 1.0;
    let g = 9.81;
    let dt = 0.001;
    let b = 0.01;
    let mut x_k = vec![std::f64::consts::FRAC_2_PI, 0.0];
    let propogate = make_propogate_euler(l, g, b, dt);

    let mut theta_values = Vec::new();
    let mut d_theta_values = Vec::new();

    for _ in 0..10000 {
        x_k = propogate(&x_k);
        theta_values.push(x_k[0]);
        d_theta_values.push(x_k[1]);
    }

    println!("Finished the program. The plot was saved as plot.png.");
    return plot(theta_values, d_theta_values);
}
