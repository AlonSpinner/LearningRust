// use crate::{make_propogate_euler, make_propogate_rk4, plot_theta_vecs, draw_3d};

fn main() {
    let pi : f64 = std::f64::consts::PI;
    let tau : f64 = std::f64::consts::TAU;
    let r : f64 = 1.0; // m
    let m : f64 = 1.0; // kg
    let k : f64 = 1.0; // N/m
    let c : f64 = 1.0; // N/(m/s)
    let n : usize = 5; // number of masses
    let dt : f64 = 0.01; // seconds
    let max_time : f64 = tau / (k/m).sqrt();
    let iterations : usize = (max_time / dt) as usize;

    //build model
    let propogate_rk4 = make_propogate_rk4(x_0, model);

    // let mut x_k = Vec::with_capacity(iterations);
    // let mut t_k = Vec::with_capacity(iterations);

    // let mut t = 0.0;
    // for _ in 1..iterations {
    //     t += dt;
    //     x_k_rk4 = propogate_rk4(t, &x_k_rk4);
        
    //     theta_euler_values.push(x_k_euler[0]);
    //     theta_rk4_values.push(x_k_rk4[0]);
    //     time_values.push(t);
    // }

    // //make a 3d drawing
    // draw_3d(&time_values, &theta_values[1], l as f32);
    
    println!("Finished the program.");

}
