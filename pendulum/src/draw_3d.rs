use three_d::*;
use crate::math::lerp1d;

pub fn draw_3d(time_vec : &Vec<f64> ,theta_vec: &Vec<f64>) {
    //vectors passed by reference so we dont take ownership of them
    //vectors are cloned so we can move them into the closure in set_animation
    let time_vec = time_vec.clone();
    let theta_vec = theta_vec.clone();

    let window = Window::new(WindowSettings {
        title: "Pendulum".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, -5.0, 0.0),
        vec3(0.0, -2.0, 0.0),
        vec3(0.0, 0.0, 1.0),
        degrees(90.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    //the sphere object
    let mut sphere = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(32)),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 200,
                },
                ..Default::default()
            },
        ),
    );
    sphere.set_transformation(Mat4::from_scale(0.2));
    sphere.set_animation(move |time| {
        let interpolated_value = lerp1d(time as f64, &time_vec, &theta_vec);
        let theta = interpolated_value as f32;
        let r = 10.0; // radius of the circle
        let x = r * theta.sin();
        let z = r- r * theta.cos();
        Mat4::from_translation(vec3(x, 0.0, z))
    });
    
    //more objects
    let axes = Axes::new(&context, 0.1, 2.0);
    let light0 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));


    let start = std::time::Instant::now();
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);
        sphere.animate(start.elapsed().as_secs_f32());

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
