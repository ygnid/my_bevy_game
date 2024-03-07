use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform , player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };
        let mut direction = Vec3::ZERO;

        //forward
        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
        }
        //back
        if keys.pressed(KeyCode::S) {
            direction += cam.back();
        }
        //left
        if keys.pressed(KeyCode::A) {
            direction += cam.left();
        }
        //back
        if keys.pressed(KeyCode::D) {
            direction += cam.right();
        }
        // println!("direction: {}", direction);

        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        // println!("delta_seconds: {}", time.delta_seconds());
        player_transform.translation += movement;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshs.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        },
        Speed(5.0),
        Player,
    );
    commands.spawn(player);
}
