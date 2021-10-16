use std::f32::consts::PI;

use bevy::prelude::*;
//use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_gizmos::{Axis, *};
use smallvec::SmallVec;

struct AnimationTime {
    time: f32,
    speed: f32,
}

impl Default for AnimationTime {
    fn default() -> Self {
        Self {
            time: 0.0,
            speed: 1.0,
        }
    }
}

// struct AnimationResource {
//     cube_size: TrackVariableLinear<Vec3>,
//     capsule_height_radius: TrackVariableLinear<Vec2>,
// }

fn main() {
    // let animation_resource = AnimationResource {
    //     cube_size: TrackVariableLinear::new(
    //         vec![0.0, 0.333, 0.666, 1.0],
    //         vec![
    //             Vec3::splat(1.0),
    //             Vec3::new(1.75, 0.25, 1.75),
    //             Vec3::new(0.25, 1.75, 0.25),
    //             Vec3::splat(1.0),
    //         ],
    //     ),
    //     capsule_height_radius: TrackVariableLinear::new(
    //         vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0],
    //         vec![
    //             Vec2::new(0.75, 0.5),
    //             Vec2::new(0.0, 0.5),
    //             Vec2::new(0.0, 0.75),
    //             Vec2::new(1.25, 0.75),
    //             Vec2::new(0.75, 0.75),
    //             Vec2::new(0.75, 0.5),
    //         ],
    //     ),
    // };

    App::new()
        // .insert_resource(animation_resource)
        .add_plugins(DefaultPlugins) // Default Bevy plugins.
        .add_plugin(GizmosPlugin)
        // .add_plugin(NoCameraPlayerPlugin)
        // .insert_resource(MovementSettings {
        //     sensitivity: 0.00012,
        //     speed: 12.0,
        // })
        .add_startup_system(setup.system())
        .add_startup_system(persistent_gizmos.system())
        .add_system(immediate_mode_gizmos_system.system())
        // .add_system(animation.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn().insert_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.spawn().insert_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    //.insert(FlyCam);
}

fn persistent_gizmos(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn().insert_bundle(GizmoBundle {
        transform: Transform::from_xyz(-4.0, 1.5, 0.0),
        gizmo: Gizmo {
            shape: GizmoShape::Empty { radius: 1.0 },
            wireframe: Color::rgba(1.0, 1.0, 0.0, 1.0),
            color: Color::rgba(0.6, 0.8, 0.2, 0.2),
        },
        ..Default::default()
    });

    commands.spawn().insert_bundle(GizmoBundle {
        transform: Transform::from_xyz(-2.0, 1.5, 0.0),
        gizmo: Gizmo {
            shape: GizmoShape::Billboard {
                texture: Some(assets.load("bevy.png")),
                size: 0.5,
            },
            wireframe: Color::WHITE, // Billboard doesn't have a gizmo
            color: Color::WHITE,
        },
        ..Default::default()
    });

    commands
        .spawn()
        .insert_bundle(GizmoBundle {
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            gizmo: Gizmo {
                shape: GizmoShape::Cube {
                    size: Vec3::splat(0.5),
                },
                wireframe: Color::rgb_linear(1.0, 0.0, 0.0),
                color: Color::rgba_linear(1.0, 0.2, 0.0, 0.2),
            },
            ..Default::default()
        })
        .insert(AnimationTime {
            speed: 0.2,
            ..Default::default()
        });

    // commands.spawn(GizmoBundle {
    //     transform: Transform::from_xyz(2.0, 1.5, 0.0),
    //     gizmo: Gizmo {
    //         shape: GizmoShape::Circle { radius: 0.5 },
    //         wireframe: Color::WHITE,
    //         color: Color::WHITE,
    //     },
    //     ..Default::default()
    // });

    commands.spawn().insert_bundle(GizmoBundle {
        transform: Transform::from_xyz(4.0, 1.5, 0.0),
        gizmo: Gizmo {
            shape: GizmoShape::Sphere { radius: 0.5 },
            wireframe: Color::rgb_linear(0.0, 0.0, 1.0),
            color: Color::rgba_linear(0.1, 0.2, 0.9, 0.2),
        },
        ..Default::default()
    });

    commands.spawn().insert_bundle(GizmoBundle {
        transform: Transform::from_xyz(-4.0, -1.5, 0.0),
        gizmo: Gizmo {
            shape: GizmoShape::Hemisphere { radius: 0.5 },
            wireframe: Color::rgb_linear(1.0, 0.0, 1.0),
            color: Color::rgba_linear(0.6, 0.0, 0.6, 0.2),
        },
        ..Default::default()
    });

    commands.spawn().insert_bundle(GizmoBundle {
        transform: Transform::from_xyz(-2.0, -1.5, 0.0),
        gizmo: Gizmo {
            shape: GizmoShape::Cylinder {
                radius: 0.5,
                height: 1.0,
            },
            wireframe: Color::WHITE,
            color: Color::WHITE,
        },
        ..Default::default()
    });

    commands
        .spawn()
        .insert_bundle(GizmoBundle {
            transform: Transform::from_xyz(0.0, -1.5, 0.0),
            gizmo: Gizmo {
                shape: GizmoShape::Capsule {
                    radius: 0.5,
                    height: 1.0,
                    axis: Axis::Y,
                },
                wireframe: Color::LIME_GREEN,
                color: {
                    let mut temp = Color::LIME_GREEN;
                    temp.set_a(0.1);
                    temp
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AnimationTime {
            speed: 0.05,
            ..Default::default()
        });

    // commands.spawn(GizmoBundle {
    //     transform: Transform::from_xyz(2.0, -1.5, 0.0),
    //     gizmo: Gizmo {
    //         shape: GizmoShape::Mesh {
    //             mesh: ???,
    //         },
    //         wireframe: Color::WHITE,
    //         color: Color::WHITE,
    //     },
    //     ..Default::default()
    // });
}

fn immediate_mode_gizmos_system(
    mut time_tracker: Local<f32>,
    time: Res<Time>,
    gizmos: Res<Gizmos>,
) {
    gizmos.draw(!0, |mut context| {
        let t = time.seconds_since_startup() as f32;
        if t < *time_tracker + 0.1 {
            return;
        }
        *time_tracker = t;

        // Generate line
        let points = (0..32)
            .into_iter()
            .map(|i| i as f32 / 33.0)
            .map(|x| Vec3::new(2.0 * x - 1.0, (PI * 1.5 * x + t).sin(), 0.0))
            .collect::<SmallVec<_>>();

        // Start drawing
        context
            // Position the line
            .push_matrix(Transform::from_xyz(4.0, -1.5, 0.0))
            // // Set it's color
            // .with_wireframe(Color::lerp(
            //     &Color::RED,
            //     &Color::SEA_GREEN,
            //     (t * 0.3).fract(),
            // ))
            // Draw line with a duration of 0.5 seconds
            .line_list(points, 2.0);
    });
}

// // Just animations
// fn animation(
//     time: Res<Time>,
//     animation_resource: Res<AnimationResource>,
//     mut query: Query<(&mut Transform, &mut Gizmo, Option<&mut AnimationTime>)>,
// ) {
//     let theta = Quat::from_rotation_y(std::f32::consts::PI * 0.1 * time.delta_seconds());
//     for (mut transform, mut gizmo, animation) in query.iter_mut() {
//         transform.rotation = theta * transform.rotation;

//         // Animate
//         if let Some(mut animation) = animation {
//             let mut t = animation.time + time.delta_seconds() * animation.speed;
//             if t > 1.0 {
//                 t = t.fract();
//             }
//             animation.time = t;

//             match gizmo.shape {
//                 // GizmoShape::Empty { radius } => {}
//                 // GizmoShape::Billboard { texture, size } => {}
//                 GizmoShape::Cube { .. } => {
//                     gizmo.shape = GizmoShape::Cube {
//                         size: animation_resource.cube_size.sample(t),
//                     };
//                 }
//                 // GizmoShape::Circle { radius } => {}
//                 // GizmoShape::Sphere { radius } => {}
//                 // GizmoShape::Hemisphere { radius } => {}
//                 // GizmoShape::Cylinder { radius, height } => {}
//                 GizmoShape::Capsule { .. } => {
//                     let v = animation_resource
//                         .capsule_height_radius
//                         .sample((t * 3.0).fract());
//                     // let axis = animation_resource.capsule_axis.sample(t);
//                     let axis = if t < 0.333 {
//                         Axis::Y
//                     } else if t < 0.666 {
//                         Axis::X
//                     } else if t < 1.0 {
//                         Axis::Z
//                     } else {
//                         Axis::Y
//                     };
//                     gizmo.shape = GizmoShape::Capsule {
//                         radius: v.y,
//                         height: v.x,
//                         axis,
//                     };
//                 }
//                 // GizmoShape::Mesh { mesh } => {}
//                 _ => {}
//             }
//         }
//     }
// }
