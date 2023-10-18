use std::ops::{Sub,Mul};
use num::complex:: Complex64;
use crate::matrix::Matrix33;
use crate::vector::V3;
use crate::EPSILON;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct SO3 {
    pub R: Matrix33,
}

impl SO3 {
    pub fn new(matrix: Matrix33) -> SO3 {
        if (matrix.det() < 0.0) || !matrix.is_orthogonal(None){
            //orthogonal matrices have determinant 1 or -1
            panic!("Matrix is not orthogonal");}
        SO3 {
            R: matrix,
        }
    }

    pub fn identity() -> SO3 {
        SO3 {
            R: Matrix33::identity(),
        }
    }

    pub fn default() -> SO3 {
        Self::identity()
    }

    pub fn inverse(&self) -> SO3 {
        SO3 {
            R: self.R.transpose(),
        }
    }

    #[allow(non_snake_case)]
    pub fn Exp(tau : V3) -> SO3 {
        let th = tau.norm();
        let theta = tau * th.recip();
        if th < EPSILON {
            return SO3::new(Matrix33::identity());
        } else {
            let hat = SO3::hat(theta);
            let hat2 = hat * hat;
            let R = Matrix33::identity() + th.sin() * hat  + (1.0 - th.cos()) * hat2;
            return SO3::new(R);
        }
    }

    #[allow(non_snake_case)]
    pub fn Log(g : SO3) -> V3 {
        let theta = ((g.R.trace() - 1.0) / 2.0).acos();
        if theta < EPSILON {
            return V3::new([0.0, 0.0, 0.0])
        } else {
            return SO3::vee(g.R - g.R.transpose()) * (theta / (2.0 * theta.sin()));
        }
    }

    fn vee(m33 : Matrix33) -> V3 {
        V3::new([m33[2][1], m33[0][2], m33[1][0]])
    }

    fn hat(v3 : V3) -> Matrix33 {
        Matrix33::new([[0.0, -v3[2], v3[1]],
                       [v3[2], 0.0, -v3[0]],
                       [-v3[1], v3[0], 0.0]])
    }
}

impl Mul for SO3 {
    type Output = SO3;
    fn mul(self, rhs: SO3) -> Self::Output {
        SO3 {
            R: self.R * rhs.R,
        }
    }
}

impl Mul<V3> for SO3 {
    type Output = V3;
    fn mul(self, rhs: V3) -> Self::Output {
        self.R * rhs
    }
}

pub struct SphericalPoint {
    r : f64,
    pitch : f64, //measured from the x axis, around the y axis
    yaw : f64, //measured from the x axis, around the z axis
}
impl SphericalPoint {
    pub fn new(r : f64, pitch : f64, yaw : f64) -> Self{
        SphericalPoint {r : r, pitch : pitch, yaw : yaw}
    }

    pub fn arcdistance(&self, other : &Self) -> f64 {
        //geodesic distance on a sphere between two points
        let v1 = self.xyz();
        let v2 = other.xyz();
        let dot_product = v1[0]*v2[0] + v1[1]*v2[1] + v1[2]*v2[2];
        let angle = dot_product.acos();
        angle * self.r
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
