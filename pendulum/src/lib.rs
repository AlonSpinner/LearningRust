mod math;
mod plot_2d;
mod draw_3d;

pub use math::make_propogate_euler;
pub use math::make_propogate_rk4;
pub use plot_2d::plot_theta_vecs;
pub use draw_3d::draw_3d;