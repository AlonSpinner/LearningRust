use three_d::*;
use crate::math::{lerp1d, SphericalPoint};
use std::sync::Arc;

pub fn draw_3d(timestamps : &Vec<f64> ,points : &Vec<Vec<SphericalPoint>>, r : f32) {
    //vectors passed by reference so we dont take ownership of them
    //vectors are cloned so we can move them into the closure in set_animation
     // Now we just clone Arc references, which is cheap
    //  let time_vec_arc = Arc::new(time_vec.clone());
    //  let theta_vec_arc = Arc::new(theta_vec.clone());

    let window = Window::new(WindowSettings {
        title: "Sphere_Springs".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(3.0*r, 3.0*r, 3.0*r),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 0.0, 1.0),
        degrees(90.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let mut sphere = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(32)),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 200,
                },
                ..Default::default()
            },
        ),
    );
    // let time_vec_clone = Arc::clone(&time_vec_arc);
    // let theta_vec_clone = Arc::clone(&theta_vec_arc);
    sphere.set_animation(move |time| {
        Mat4::from_translation(vec3(0.0, 0.0, 0.0))
    });

    
    //more objects
    let axes = Axes::new(&context, 0.1, 2.0);
    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));


    let start = std::time::Instant::now();
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);
        sphere.animate(start.elapsed().as_secs_f32());
        // points.animate(start.elapsed().as_secs_f32());

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
            .render(
                &camera,
                sphere
                .into_iter()
                .chain(&axes),
                &[&light0, &light1],
            );

        FrameOutput::default()
    });
}
