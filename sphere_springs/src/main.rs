use sphere_springs::math::{SphericalPoint,RK4};
use sphere_springs::draw_3d::draw_3d;
use num::complex:: Complex64;

fn main() {
    const PI : f64 = std::f64::consts::PI;
    const TAU : f64 = std::f64::consts::TAU;
    const R : f64 = 1.0; // m
    const M : f64 = 1.0; // kg
    const K : f64 = 1.0; // N/m
    const C : f64 = 3.0; // N/(m/s)
    const N : usize = 4; // number of masses
    
    let dt : f64 = 0.001; // seconds
    let max_time : f64 = 10.0 * TAU / (K/M).sqrt();
    let iterations : usize = (max_time / dt) as usize;

    fn free_length(dx : f64, theta0 : f64) -> f64 {
        (Complex64::new(0.0, dx).exp() * Complex64::new(0.0, theta0).exp()).ln().im
    }

    // build model
    #[allow(unused_variables)]
    fn f(t : f64, x : &Vec<f64>) -> Vec<f64> {
        //x - [pitch, theta, pitch_dot, theta_dot]_1, [pitch, theta, pitch_dot, theta_dot]_2, ...
    
        let mut x_dot: Vec<f64> = vec![0.0; 4*N];
        for i in 0..N {
            let mut force = [0.0, 0.0];
            for j in 0..N {
                if i == j {continue};
                //compute force proportional to distance and velocity, in tangen space of x_i
                let dx = SphericalPoint::new(R,x[4*j],x[4*j+1]) -
                                    SphericalPoint::new(R,x[4*i],x[4*i+1]);
                // println!("{:?}", dx);
                let mut dv = [0.0;2];
                dv[0] = x[4*j+2] - x[4*i+2];
                dv[1] = x[4*j+3] - x[4*i+3];

                force[0] = -K * free_length(dx[0], 0.0) + C * dv[0];
                force[1] = -K * free_length(dx[1], PI) + C * dv[1];
            }
            let a = [force[0]/M, force[1]/M];
            
            x_dot[4*i] = x[4*i+2];
            x_dot[4*i+1] = x[4*i+3];
            x_dot[4*i+2] = a[0];
            x_dot[4*i+3] = a[1];
        }
        return x_dot;
    }

    fn x_2_positions(x : &Vec<f64>) -> Vec<[f32;3]> {
        //x - [pitch, theta, pitch_dot, theta_dot]_1, [pitch, theta, pitch_dot, theta_dot]_2, ...
        //positions - [x,y,z]_1, [x,y,z]_2, ...
        let mut positions: Vec<[f32;3]> = Vec::with_capacity(N);
        for i in 0..N {
            let tmp = SphericalPoint::new(R, x[4*i], x[4*i+1]).xyz();
            positions.push([tmp[0] as f32, tmp[1] as f32, tmp[2] as f32]);
        }
        positions        
    }

    let rk4 = RK4::new(dt, f);
    // build initial state
    let mut x_k : Vec<f64> = vec!(0.0; 4 * N); //initial state
    for i in 0..N {
        //random pitch theta
        x_k[4*i] = PI/2.0 * (2.0 * rand::random::<f64>() - 1.0);
        x_k[4*i+1] = PI * (2.0 * rand::random::<f64>() - 1.0);
    }
    let mut timestamps : Vec<f32> = Vec::with_capacity(iterations);
    let mut positions : Vec<Vec<[f32;3]>> = Vec::with_capacity(iterations);

    let mut t = 0.0;
    for _ in 1..iterations {
        t += dt;
        x_k = rk4.propogate(t, &x_k);
        
        timestamps.push(t as f32);
        positions.push(x_2_positions(&x_k));
    }

    // //make a 3d drawing
    draw_3d(&timestamps, &positions, R as f32);
    println!("Finished the program.");

}
