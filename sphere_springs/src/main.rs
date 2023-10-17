use sphere_springs::math::{SphericalPoint,RK4};
use sphere_springs::draw_3d::draw_3d;

fn main() {
    const PI : f64 = std::f64::consts::PI;
    const TAU : f64 = std::f64::consts::TAU;
    const R : f64 = 1.0; // m
    const M : f64 = 1.0; // kg
    const K : f64 = 1.0; // N/m
    const C : f64 = 1.0; // N/(m/s)
    const N : usize = 5; // number of masses
    
    let dt : f64 = 0.01; // seconds
    let max_time : f64 = 10.0 * TAU / (K/M).sqrt();
    let iterations : usize = (max_time / dt) as usize;

    // build model
    fn f(t : f64, x : &Vec<f64>) -> Vec<f64> {
        //x - [pitch, theta, pitch_dot, theta_dot]_1, [pitch, theta, pitch_dot, theta_dot]_2, ...
        assert!(x.len() == 4 * N);
        
        let mut sph_x: Vec<SphericalPoint> = Vec::with_capacity(2*N);
        //sph_x - [x, v]_1, [x, v]_2, ...
        for i in 0..N {
            sph_x.push(SphericalPoint::new(R, x[4*i], x[4*i+1]));
            sph_x.push(SphericalPoint::new(R, x[4*i+2], x[4*i+3]));
        }

        let mut x_dot: Vec<f64> = vec![0.0; 4*N];
        for i in 0..N {
            let mut force = [0.0, 0.0];
            for j in 0..N {
                if i == j {continue};
                //compute force proportional to distance and velocity, in tangen space of x_i
                let dx = sph_x[2*j] - sph_x[2*i];
                let dv  = sph_x[2*j+1] - sph_x[2*i+1];
                force[0] += K * dx[0] + C * dv[0];
                force[1] += K * dx[1] + C * dv[1];

            let a = [force[0]/M, force[1]/M];
            x_dot[i] = x[4*i+2];
            x_dot[i+1] = x[4*i+3];
            x_dot[i+2] = a[0];
            x_dot[i+3] = a[1];
            
            }
        }
        return x_dot;
    }

    fn x_2_positional_sph(x : &Vec<f64>) -> Vec<SphericalPoint> {
        let mut positional_sph_x: Vec<SphericalPoint> = Vec::with_capacity(N);
        for i in 0..N {
            positional_sph_x.push(SphericalPoint::new(R, x[4*i], x[4*i+1]));
        }
        positional_sph_x
    }

    let rk4 = RK4::new(dt, f);

    let mut x_k : Vec<f64> = vec!(0.0; 4 * N); //initial state
    let mut timestamps : Vec<f64> = Vec::with_capacity(iterations);
    let mut positional_sph_x : Vec<Vec<SphericalPoint>> = Vec::with_capacity(iterations);

    let mut t = 0.0;
    for _ in 1..iterations {
        t += dt;
        x_k = rk4.propogate(t, &x_k);
        
        timestamps.push(t);
        positional_sph_x.push(x_2_positional_sph(&x_k));
    }

    // //make a 3d drawing
    draw_3d(&timestamps, &positional_sph_x, R as f32);
    println!("Finished the program.");

}
