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
    d2_theta_k = -g/l * sin(theta_k) - b * d_theta_k
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




