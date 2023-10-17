use three_d::*;
use crate::math::lerp1d;

#[derive(Default, Clone)]
struct XyzHistory {
    x : Vec<f32>,
    y : Vec<f32>,
    z : Vec<f32>,
}

pub fn draw_3d(timestamps : &Vec<f32> ,points_by_time : &Vec<Vec<[f32;3]>>, r : f32) {
    /*
    points_by_time - outer vector is time, inner vector is points
     */
    let n = points_by_time.len();
    let m = points_by_time[0].len();

    //transpose points_by_time so xyz data is by index
    let mut points_by_index: Vec<XyzHistory> = vec![XyzHistory::default(); n];
    for p in points_by_time {
        for i in 0..m {
            points_by_index[i].x.push(p[i][0]);
            points_by_index[i].y.push(p[i][1]);
            points_by_index[i].z.push(p[i][2]);
        }
    }

    // let arc_timestamps = Arc::new(timestamps.clone());

    fn interp_location(time : f32, timestamps : &Vec<f32>, history : &XyzHistory) -> [f32;3] {
        let x = lerp1d(time, &timestamps, &history.x);
        let y = lerp1d(time, &timestamps, &history.y);
        let z = lerp1d(time, &timestamps, &history.z);
        [x,y,z]
    }

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

    let mut mesh = CpuMesh::sphere(32);
    mesh.transform(&Mat4::from_scale(r)).unwrap();
    let sphere = Gm::new(
        Mesh::new(&context, &mesh),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 100,
                },
                ..Default::default()
            },
        ),
    );

    let mut points = Vec::with_capacity(n);
    for i in 0..m {
        let mut mesh = CpuMesh::sphere(32);
        mesh.transform(&Mat4::from_scale(0.1 * r)).unwrap();
        let mut point = Gm::new(
            Mesh::new(&context, &mesh),
            PhysicalMaterial::new_transparent(
                &context,
                &CpuMaterial {
                    albedo: Srgba::BLUE,
                    ..Default::default()
                },
            ),
        );
        let tmp_timestamps = timestamps.clone();
        let tmp_history = points_by_index[i].clone();
        point.set_animation(move |time| {
            let xyz = interp_location(time, &tmp_timestamps, &tmp_history);
            Mat4::from_translation(vec3(xyz[0], xyz[1], xyz[2]))
        });
        points.push(point)
        
    }

    //more objects
    let axes = Axes::new(&context, 0.05 * r, 0.5 * r);
    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE,
                                                         &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE,
                                                         &vec3(0.0, 0.5, 0.5));


    let start = std::time::Instant::now();
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);
        for p in &mut points {
            p.animate(start.elapsed().as_secs_f32());
        }

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
            .render(
                &camera,
                sphere.into_iter()
                .chain(&axes)
                .chain(points.iter().flatten()),
                &[&light0, &light1],
            );

        FrameOutput::default()
    });
}
