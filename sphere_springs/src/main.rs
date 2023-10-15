use sphere_springs::math::{RS1S1,RK4};

fn main() {
    const PI : f64 = std::f64::consts::PI;
    const TAU : f64 = std::f64::consts::TAU;
    const R : f64 = 1.0; // m
    const M : f64 = 1.0; // kg
    const K : f64 = 1.0; // N/m
    const C : f64 = 1.0; // N/(m/s)
    const N : usize = 5; // number of masses
    
    let dt : f64 = 0.01; // seconds
    let max_time : f64 = TAU / (K/M).sqrt();
    let iterations : usize = (max_time / dt) as usize;

    // build model
    fn f(t : f64, x : &Vec<RS1S1>) -> Vec<RS1S1> {
        let x_dot = Vec::with_capacity(2 * N);

        for i in 0..N {
            let force = 0.0;
            for j in 0..N {
                if i == j {continue};
                force += K * (x[i] - x[j]).arclength() +
                         C * (x[2*i+1] - x[2*j+1]).arclength();
            

            x_dot[i] = x[2*i+1];
            x_dot[2*i+1] = force / M;
            }
        }
        return x_dot;
    }
    let rk4 = RK4::new(dt, f);

    // let mut x_k = Vec<>::with_capacity(iterations);
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
