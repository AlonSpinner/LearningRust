use std::ops::Sub;
use num::complex:: Complex64;

#[derive(Debug, Copy, Clone)]
pub struct SphericalPoint {
    r : f64,
    pitch : f64, //measured from the x axis, around the y axis
    yaw : f64, //measured from the x axis, around the z axis
}

pub fn cross(a : &[f64;3], b : &[f64;3]) -> [f64;3] {
    [a[1]*b[2] - a[2]*b[1],
     a[2]*b[0] - a[0]*b[2],
     a[0]*b[1] - a[1]*b[0]]
}

pub fn dot(a : &[f64;3], b : &[f64;3]) -> f64 {
    a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
}

pub fn norm(a : &[f64;3]) -> f64 {
    (a[0]*a[0] + a[1]*a[1] + a[2]*a[2]).sqrt()
}

pub fn normalize(a : &[f64;3]) -> [f64;3] {
    let norm = norm(a);
    [a[0]/norm, a[1]/norm, a[2]/norm]
}

impl SphericalPoint {
    pub fn new(r : f64, pitch : f64, yaw : f64) -> Self{
        SphericalPoint {r : r, pitch : pitch, yaw : yaw}
    }

    pub fn axis_angle_arc(&self, other : &Self) -> ([f64;3], f64, f64) {
        //angle between two points on a sphere
        let v1 = self.xyz();
        let v2 = other.xyz();
        let cross_product = cross(&v1, &v2);
        let axis = normalize(&cross_product);
        let angle = dot(&normalize(&v1),&normalize(&v2)).acos();
        let arc = self.r * angle;
        (axis, angle, arc)
    }

    pub fn xyz(&self) -> [f64;3] {
        let x = self.r * self.pitch.cos() * self.yaw.cos();
        let y = self.r * self.pitch.cos() * self.yaw.sin();
        let z = self.r * self.pitch.sin();
        return [x,y,z]
    }
}

impl Sub for SphericalPoint {
    type Output = [f64;2];

    fn sub(self, other: Self) -> Self::Output {
        let c_pitch = (Complex64::new(0.0, self.pitch).exp() * 
                                    Complex64::new(0.0, -other.pitch).exp()).ln();
        let c_yaw = (Complex64::new(0.0, self.yaw).exp() * 
                                    Complex64::new(0.0, -other.yaw).exp()).ln();
        assert!(c_pitch.re.abs() < 1e-10, "real c_pitch is not zero");
        assert!(c_yaw.re.abs() < 1e-10, "real c_yaw is not zero");
        assert!(self.r == other.r, "Cannot subtract two points with different radii");

        [self.r * c_pitch.im, self.r * c_yaw.im]
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

    pub fn propogate(&self, t : f64, x : &Vec<f64>) -> Vec<f64>{
        let n = x.len();
        let half_dt = self.dt/2.0;

        let k1 = (self.f)(t, x);
        assert_eq!(k1.len(),n, "f(t,x) must produce a vector of the same size as x");
               
        let mut x_tmp = vec![0.0; n];
        for i in 0..n {x_tmp[i] = x[i] + half_dt * k1[i];}
        let k2 = (self.f)(t + half_dt, &x_tmp);
        
        for i in 0..n {x_tmp[i] = x[i] + half_dt * k2[i];}
        let k3 = (self.f)(t + half_dt, &x_tmp);

        for i in 0..n {x_tmp[i] = x[i] + self.dt * k3[i];}
        let k4 = (self.f)(t + self.dt, &x_tmp);

        for i in 0..n {
            x_tmp[i] = x[i] + (self.dt/6.0)*(k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        x_tmp
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

#[test]
fn test_spherical_point() {
    const PI : f64 =  std::f64::consts::PI;
    let p1 = SphericalPoint::new(1.0, 0.0, -2.0 * PI);
    let p2 = SphericalPoint::new(1.0, 0.0, PI);
    
    let d = p1.arcdistance(&p2);
    assert!((d - PI).abs() < 1e-10);

    let s = p2 - p1;
    assert!(s[0] == 0.0);
    assert!((s[1] - PI).abs() < 1e-10);


    let p1 = SphericalPoint::new(1.0, PI, -PI);
    let p2 = SphericalPoint::new(1.0, PI, PI);
    let d = p1 - p2;
    assert!((d[0]-0.0).abs() < 1e-10);
    assert!((d[1]-0.0).abs() < 1e-10);
}
