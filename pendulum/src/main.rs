use pendulum::{make_propogate_euler, make_propogate_rk4, plot_theta_vecs};

fn main() {
    let l: f64 = 1.0;
    let g: f64 = 9.81;
    let dt: f64 = 0.06;
    let b: f64 = 0.5;
    let max_time: f64 = 10.0 * (2.0 * std::f64::consts::PI * (l/g).sqrt());
    let iterations : usize = (max_time / dt) as usize;

    let mut x_k_euler = vec![3.14/2.0, 0.0];
    let propogate_euler = make_propogate_euler(l, g, b, dt);

    let mut x_k_rk4 = x_k_euler.clone();
    let propogate_rk4 = make_propogate_rk4(l, g, b, dt);

    let mut theta_euler_values = Vec::with_capacity(iterations);
    let mut theta_rk4_values = Vec::with_capacity(iterations);
    let mut time_values = Vec::with_capacity(iterations);

    theta_euler_values.push(x_k_euler[0]);
    theta_rk4_values.push(x_k_rk4[0]);
    time_values.push(0.0);

    let mut t = 0.0;
    for _ in 1..iterations {
        t += dt;
        x_k_euler = propogate_euler(&x_k_euler);
        x_k_rk4 = propogate_rk4(t, &x_k_rk4);
        
        theta_euler_values.push(x_k_euler[0]);
        theta_rk4_values.push(x_k_rk4[0]);
        time_values.push(t);
    }

    //make a 2d plot
    plot_theta_vecs(time_values,
         vec!(theta_euler_values,theta_rk4_values),
         vec!("euler","rk4")).expect("plotting failed");
    
    println!("Finished the program. The plot was saved as plot.png.");
}
