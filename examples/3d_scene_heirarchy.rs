use bevy::prelude::*;
use bevy_mod_picking::{
    DebugCursorPickingPlugin, DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle,
    PickingCameraBundle,
};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            vsync: false, // Disabled for this demo to remove vsync as a source of input latency
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(DefaultPickingPlugins)
        // .add_plugin(DebugCursorPickingPlugin)
        // .add_plugin(DebugEventsPickingPlugin)
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    //         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //         ..Default::default()
    //     })
    //     .insert_bundle(PickableBundle::default());

    // cube
    let mut root = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        });
        // root.insert_bundle(PickableBundle::default());
    
        let children = [
            (-1., -1., -1.),
            (-1., -1., 1.),
            (-1., 1., 1.), 
            (-1., 1., -1.),
            (1., 1., 1.),
            (1., 1., -1.),
            (1., -1., 1.),
            (1., -1., -1.),

        ];
        for origin in children.iter() {
            println!("{:?}", origin);
            root.with_children(|parent| {
                parent.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(origin.0, origin.1, origin.2),
                    ..Default::default()
                });
                // .insert_bundle(PickableBundle::default());

            });
        }

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
        // .insert_bundle(PickingCameraBundle::default());
}


fn update (
    mut commands: Commands,
    mut parents_query: Query<(Entity, &Children)>,
    mut total_query: Query<Entity>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,   
    keyboard: Res<Input<KeyCode>>,

) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    println!("pressed");
    println!("total elements: {}", total_query.iter().len());
    let child_origins = [
            (-1., -1., -1.),
            (-1., -1., 1.),
            (-1., 1., 1.),
            (-1., 1., -1.),
            (1., 1., 1.),
            (1., 1., -1.),
            (1., -1., 1.),
            (1., -1., -1.),

        ];
    for (parent, children) in parents_query.iter_mut() {
        for child in children.iter() {
            commands.entity(*child).despawn();
        }
        for origin in child_origins.iter() {
            // println!("{:?}", origin);
            commands.entity(parent).with_children(|parent| {  
                parent.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(origin.0, origin.1, origin.2),
                    ..Default::default()
                });
                // .insert_bundle(PickableBundle::default());
            });
        }
    }
}