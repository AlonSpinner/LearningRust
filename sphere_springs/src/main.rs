use core::panic;

use sphere_springs::math::{SphericalPoint,RK4,cross, dot, normalize};
use sphere_springs::draw_3d::draw_3d;
use num::complex:: Complex64;

fn main() {
    const PI : f64 = std::f64::consts::PI;
    const TAU : f64 = std::f64::consts::TAU;
    const R : f64 = 1.0; // m
    const M : f64 = 1.0; // kg
    const K : f64 = 1.0; // N/m
    const C : f64 = 1.0; // N/(m/s)
    const N : usize = 5; // number of masses
    
    let dt : f64 = 0.001; // seconds
    let max_time : f64 = 10.0 * TAU / (K/M).sqrt();
    let iterations : usize = (max_time / dt) as usize;

    fn free_length(angle : f64, angle0 : f64) -> f64 {
        (Complex64::new(0.0, angle).exp() * Complex64::new(0.0, angle0).exp()).ln().im
    }

    // build model
    #[allow(unused_variables)]
    fn f(t : f64, x : &Vec<f64>) -> Vec<f64> {
        //x - [theta, phi, theta_dot, phi_dot]_1, [theta, phi, theta_dot, phi_dot]_2, ...
    
        let mut x_dot: Vec<f64> = vec![0.0; 4*N];
        for i in 0..N {
            let mut f_theta = 0.0;
            let mut f_phi = 0.0;
            for j in 0..N {
                if i == j {continue};
                //compute f_k
                let sph_j = SphericalPoint::new(R,x[4*j],x[4*j+1]);
                let sph_i = SphericalPoint::new(R,x[4*i],x[4*i+1]);
                let (axis, angle, arc) =  sph_i.axis_angle_arc(&sph_j);
                let tangent = cross(&axis, &normalize(&sph_i.e_r()));
                let f_tangent = K * R * free_length(angle, PI);
                f_theta += f_tangent * dot(&tangent, &sph_i.e_theta());
                f_phi += f_tangent * dot(&tangent, &sph_i.e_phi());
            }
            let theta = x[4*i];
            let phi = x[4*i+1];
            let theta_dot = x[4*i+2];
            let phi_dot = x[4*i+3];

            //compute f_d
            let v_theta = R * theta_dot;
            let v_phi = R * theta.sin() * phi_dot;
            f_theta -= C * v_theta;
            f_phi -= C * v_phi;

            //equations of motions with constant R
            // https://en.wikipedia.org/wiki/Equations_of_motion
            let theta_ddot = (f_theta/M + R*phi_dot.powi(2)*theta.sin()*theta.cos())/R;
            let phi_ddot = (f_phi/M - 2.0*R*theta_dot*phi_dot*theta.cos())/(R * theta.sin()).min(1000.0);
            
            x_dot[4*i] = theta_dot;
            x_dot[4*i+1] = phi_dot;
            x_dot[4*i+2] = theta_ddot;
            x_dot[4*i+3] = phi_ddot;
        }
        return x_dot;
    }

    fn x_2_positions(x : &Vec<f64>) -> Vec<[f32;3]> {
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

    //compute mean and std of arclength on last iteration
    let last_positions = positions[iterations-2].clone();
    let mut arclengths : Vec<f64>;
    for i in 0..N {
        for j in 0..N {
            if i == j {continue};
            let sph_i = SphericalPoint::new(R, last_positions[i][0] as f64, last_positions[i][1] as f64);
            let sph_j = SphericalPoint::new(R, last_positions[j][0] as f64, last_positions[j][1] as f64);
            let (_, _, arc) = sph_i.axis_angle_arc(&sph_j);
            if arc.is_nan() {panic!("arc is nan")};
            arclengths.push(arc);
        }
    }
    let mean_arclength = arclengths.iter().sum::<f64>() / arclengths.len() as f64;
    let std_arclength = (arclengths.iter().map(|x| (x - mean_arclength).powi(2)).sum::<f64>() / arclengths.len() as f64).sqrt();
    println!("Mean arclength: {}", mean_arclength);
    println!("Std arclength: {}", std_arclength);


    // //make a 3d drawing
    draw_3d(&timestamps, &positions, R as f32);
    println!("Finished the program.");

}
