use std::ops::Sub;
use num::complex:: Complex64;

pub struct S1S1 {
    c_pitch : Complex64,
    c_yaw : Complex64,
}
impl S1S1 {
    pub fn new(pitch : f64, yaw : f64) -> Self{
        let c_pitch = Complex64::new(0.0, pitch).exp();
        let c_yaw = Complex64::new(0.0, yaw).exp();
        S1S1 {c_pitch : c_pitch, c_yaw :c_yaw}
    }
}

impl Sub for S1S1 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        S1S1 {
            c_pitch: other.c_pitch.inv() * self.c_pitch,
            c_yaw: other.c_yaw.inv() * self.c_yaw ,
        }
    }
}

pub struct RK4<F>
where F : Fn (f64, &Vec<f64>) -> Vec<f64> {
    dt : f64,
    f : F,
}
impl<F> RK4<F> 
where F: Fn (f64, &Vec<f64>) -> Vec<f64> {
    pub fn new(dt : f64, f : F) -> Self {
        RK4 {dt : dt, f : f}
    }

    pub fn propogate(self, t : f64, x : &Vec<f64>) -> Vec<f64>{
        let n = x.len();
        let half_dt = self.dt/2.0;

        let k1 = (self.f)(t, x);
        assert_eq!(k1.len(),n, "f(t,x) must produce a vector of the same size as x");
               
        let mut x_tmp = vec![0.0; n];
        let mut k2 = vec![0.0; n];
        for i in 0..n {
            x_tmp[i] = x[i] + half_dt * k1[i];
            k2 = (self.f)(t + half_dt, &x_tmp);
        }
        
        let mut k3 = vec![0.0; n];
        for i in 0..n {
            x_tmp[i] = x[i] + half_dt * k2[i];
            k3 = (self.f)(t + half_dt, &x_tmp);
        }

        let mut k4 = vec![0.0; n];
        for i in 0..n {
            x_tmp[i] = x[i] + self.dt * k3[i];
            k4 = (self.f)(t + self.dt, &x_tmp);
        }

        for i in 0..n {
            x_tmp[i] = x[i] + (self.dt/6.0)*(k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        return x_tmp
    }
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
fn test_lerp1d() {
    let x_vec = vec![0.0, 1.0, 2.0, 3.0];
    let y_vec = vec![3.0, 2.0, 1.0, 0.0];
    let x = 0.5;
    let y = lerp1d(x, &x_vec, &y_vec);
    assert!(y == 2.5);
}
