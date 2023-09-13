mod math;
pub use math::make_propogate_euler;
pub use math::make_propogate_rk4;
pub use math::lerp1d;

mod plot_2d;
pub use plot_2d::plot_theta_vecs;

mod draw_3d;
pub use draw_3d::draw_3d;