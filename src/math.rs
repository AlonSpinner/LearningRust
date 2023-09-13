pub fn make_propogate_euler(l : f64, g : f64, b : f64, dt : f64) -> impl Fn(&Vec<f64>) -> Vec<f64> {
    move |x_k| propogate_euler(x_k, l, g, b, dt)
}

fn propogate_euler(x_k: &Vec<f64>, l : f64, g : f64, b : f64, dt : f64) -> Vec<f64> {
    /*
    x_k - state at time k [theta_k, d_theta_k]
    l - length of pendulum
    g - gravity
    b - damping
    dt - time step

    EOM:
    d^2theta/dt^2 = -g/l * sin(theta) - b * dtheta/dt
     */

    let theta_k = x_k[0];
    let d_theta_k = x_k[1];

    let d2_theta_k = -g/l * theta_k.sin() - b * d_theta_k;

    //Euler integration
    let theta_kp1 = theta_k + d_theta_k * dt;
    let d_theta_kp1 = d_theta_k + d2_theta_k * dt;

    
    let x_kp1 = vec![theta_kp1, d_theta_kp1];
    return x_kp1;
}

pub fn make_propogate_rk4(l : f64, g : f64, b : f64, dt : f64) -> impl Fn(f64, &Vec<f64>) -> Vec<f64> {
    move |t, x| propogate_rk4(t, x, l, g, b, dt)
}

fn propogate_rk4(t : f64, x: &Vec<f64>, l : f64, g : f64, b : f64, dt : f64) -> Vec<f64> {
    /*
    x - current state [theta_k, d_theta_k]
    l - length of pendulum
    g - gravity
    b - damping
    dt - time step

    EOM:
    d2_theta = -g/l * sin(theta) - b * d_theta
     */

     //define derivative function (continuous time)
     //t is here just for consistency with the RK4 algorithm
     let f = |_t: f64, x: &Vec<f64>| -> Vec<f64> {
        let theta = x[0];
        let d_theta = x[1];
        let d2_theta = -g/l * theta.sin() - b * d_theta;
        return vec![d_theta, d2_theta];
    };

    //RK4 integration
    let k1 = f(t, &x);
    
    let x4k2_0 = x[0] + dt/2.0 * k1[0]; 
    let x4k2_1 = x[1] + dt/2.0 * k1[1];
    let x4k2 = vec![x4k2_0, x4k2_1];
    let k2 = f(t + dt/2.0, &x4k2);
    
    let x4k3_0 = x[0] + dt/2.0 * k2[0];
    let x4k3_1 = x[1] + dt/2.0 * k2[1];
    let x4k3 = vec![x4k3_0, x4k3_1];
    let k3 = f(t + dt/2.0, &x4k3);

    let x4k4_0 = x[0] + dt * k3[0];
    let x4k4_1 = x[1] + dt * k3[1];
    let x4k4 = vec![x4k4_0, x4k4_1];
    let k4 = f(t + dt, &x4k4);

    let row1 = x[0] + dt/6.0 * (k1[0] + 2.0*k2[0] + 2.0*k3[0] + k4[0]);
    let row2 = x[1] + dt/6.0 * (k1[1] + 2.0*k2[1] + 2.0*k3[1] + k4[1]);
    return vec![row1, row2];
}

pub fn lerp1d<T>(x : T, x_vec : &Vec<T>, y_vec : &Vec<T>) -> T
    where T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T> 
            + std::ops::Div<Output = T> 
            + std::ops::Mul<Output = T> 
            + std::cmp::PartialOrd
            + std::marker::Copy {
    //find the two points in x_vec that are closest to x
    //interpolate between the two points
    //return the interpolated value

    if x < x_vec[0] {
        return y_vec[0];
    }
    else if x > x_vec[x_vec.len()-1] {
        return y_vec[y_vec.len()-1];
    }
    else
    {
    //find the two points in x_vec that are closest to x
    //use binary search because x_vec is sorted
    let i = match x_vec.binary_search_by(|&probe| probe.partial_cmp(&x).unwrap()) {
        Ok(index) => index,
        Err(index) => index,
    };
    let x0 = x_vec[i-1];
    let x1 = x_vec[i];
    let y0 = y_vec[i-1];
    let y1 = y_vec[i];

    //interpolate between the two points
    let y = y0 + (y1 - y0) / (x1 - x0) * (x - x0);
    return y;
    }
}

#[test]
fn test_propogate_euler() {
    let l = 1.0;
    let g = 9.81;
    let dt = 0.01;
    let b = 1.0;
    let mut x_k = vec![std::f64::consts::FRAC_2_PI, 0.0];
    //we have a stable system, test that after 10 seconds we are close to the origin
    
    let propogate = make_propogate_euler(l, g, b, dt);

    for _ in 0..1000 {
        x_k = propogate(&x_k);
    }
    //assert that we are close to the origin
    assert!(x_k[0].abs() < 0.01);
    assert!(x_k[1].abs() < 0.01);
}

#[test]
fn test_propogate_rk4() {
    let l = 1.0;
    let g = 9.81;
    let dt = 0.01;
    let b = 1.0;
    
    let mut t = 0.0;
    let mut x_k = vec![std::f64::consts::FRAC_2_PI, 0.0];
    //we have a stable system, test that after 10 seconds we are close to the origin

    let propogate = make_propogate_rk4(l, g, b, dt);

    for _ in 0..1000 {
        t += dt;
        x_k = propogate(t, &x_k);
    }
}

#[test]
fn test_lerp1d() {
    let x_vec = vec![0.0, 1.0, 2.0, 3.0];
    let y_vec = vec![3.0, 2.0, 1.0, 0.0];
    let x = 0.5;
    let y = lerp1d(x, &x_vec, &y_vec);
    assert!(y == 2.5);
}
