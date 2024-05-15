use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::query,
    input::keyboard,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
struct CameraCreator;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity {
    value: Vec3,
}

#[derive(Component)]
struct Wall;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        CameraCreator,
    ));

    let player_color = Color::rgb(0.25, 0.0, 1.0);
    let wall_color = Color::rgb(0.5, 0.5, 0.5);
    let ground_range = 10;

    for i in -ground_range..ground_range {
        let wall = commands
            .spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0))),
                material: materials.add(wall_color),
                transform: Transform::from_xyz(i as f32 * 50.0, -100.0, 0.0),
                ..default()
            })
            .id();
        commands.entity(wall).insert(Wall);
    }

    let player = commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0))),
            material: materials.add(player_color),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .id();
    commands.entity(player).insert((
        Player,
        Velocity {
            value: Vec3::new(0.0, 0.0, 0.0),
        },
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform, &mut Velocity)>,
) {
    let movement_speed = 3.5;
    let jump_force = 5.0;

    let (_, mut transform, mut velocity) = query.get_single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.value.x = -movement_speed;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.value.x = movement_speed;
    } else {
        velocity.value.x = 0.0;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        velocity.value.y = jump_force;
    }

    velocity.value.y -= 10.0 * time.delta_seconds();
    if velocity.value.y < -10.0 {
        velocity.value.y = -10.0;
    }
    transform.translation += velocity.value * time.delta_seconds() * 100.0;
}

fn collide_with_player(
    mut player: Query<(&Player, &mut Transform, &mut Velocity)>,
    walls: Query<(&Wall, &Transform), Without<Player>>,
) {
    let half_size = 25.0;

    let (_, mut player_transform, mut velocity) = player.get_single_mut().unwrap();

    for (_, wall_transform) in walls.iter() {
        let player_bottom = player_transform.translation.y - half_size;
        let player_top = player_transform.translation.y + half_size;
        let player_right = player_transform.translation.x + half_size;
        let player_left = player_transform.translation.x - half_size;

        let wall_bottom = wall_transform.translation.y - half_size;
        let wall_top = wall_transform.translation.y + half_size;
        let wall_left = wall_transform.translation.x - half_size;
        let wall_right = wall_transform.translation.x + half_size;

        let is_colliding = (player_transform.translation.x > wall_left
            && player_transform.translation.x < wall_right)
            || (player_transform.translation.y > wall_bottom
                && player_transform.translation.y < wall_top);

        if is_colliding {
            // moving right
            if velocity.value.x > 0.0 {
                if player_right > wall_left {
                    player_transform.translation.x = wall_left - half_size;
                }
                velocity.value.x = 0.0;
            }
            // moving left
            if velocity.value.x < 0.0 {
                if player_left < wall_right {
                    player_transform.translation.x = wall_right + half_size;
                }
                velocity.value.x = 0.0;
            }
            // moving up
            if velocity.value.y > 0.0 {
                if player_top > wall_bottom {
                    player_transform.translation.y = wall_bottom - half_size;
                }
                velocity.value.y = 0.0;
            }
            // moving down
            if velocity.value.y < 0.0 {
                if player_bottom < wall_top {
                    player_transform.translation.y = wall_top + half_size;
                }
                velocity.value.y = 0.0;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LogDiagnosticsPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_player, collide_with_player.after(move_player)),
        )
        .run();
}
