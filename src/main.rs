use std::str::FromStr;

use bevy::{log::LogPlugin, prelude::*};
use rand::prelude::*;

fn main() {
    App::new()
        //Disable the defaults
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spammy::print_trash)
        .add_systems(Update, some_queries.run_if(run_once()))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 50.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    let mut rng = rand::thread_rng();

    // Create spiral galaxy
    for i in 0..10000 {
        let t = i as f32 * 0.005;
        let r = 20.0 * t.sqrt();
        let theta = t * 2.5;

        let x = r * theta.cos();
        let z = r * theta.sin();
        let y = (rng.gen::<f32>() - 0.5) * 2.0; // Add some vertical spread

        let size = (0.05 + rng.gen::<f32>() * 0.1) * (1.0 - t / 50.0); // Smaller stars towards the edge
        let brightness = 1.0 - t / 50.0; // Dimmer stars towards the edge

        let color = Color::srgb(brightness, brightness * 0.8 + 0.2, brightness * 0.6 + 0.4);
        let emissive = Color::srgb(
            brightness * 5.0,
            (brightness * 0.8 + 0.2) * 5.0,
            (brightness * 0.6 + 0.4) * 5.0,
        );

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere { radius: size })),
            material: materials.add(StandardMaterial {
                base_color: color,
                emissive: emissive.into(), // Make stars glow
                ..default()
            }),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        });
    }
}

fn some_queries(transformers: Query<&Transform>) {}

mod spammy {

    pub fn print_trash() {
        for i in 0..10 {
            some_function(i);
        }
    }
    fn some_function(i: u8) {
        if i == 3 {}
    }
}
