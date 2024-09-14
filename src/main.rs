use bevy::{prelude::*, window::WindowResolution};

// Component to mark our player rectangle
#[derive(Component)]
struct Paddle;

struct WindowSize;

impl WindowSize {
    const WIDTH: f32 = 900.0;
    const HEIGHT: f32 = 600.0;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WindowSize::WIDTH, WindowSize::HEIGHT),
                title: "My Bevy Game".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, paddle_movement)
        .run();
}

fn setup(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();

    commands.spawn(Camera2dBundle::default());

    let paddle_width = 100.0;
    let paddle_height = 20.0;
    let padding = 40.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1., 0., 0.),
                custom_size: Some(Vec2::new(paddle_width, paddle_height)),
                ..default()
            },
            transform: Transform::from_xyz(
                0.0,
                -window.height() / 2.0 + paddle_height / 2.0 + padding,
                0.0
            ),
            ..default()
        },
        Paddle,
    ));
}

fn paddle_movement(
    mut query: Query<(&mut Transform, &Sprite), With<Paddle>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window: Query<&Window>,
) {
    let (mut transform, sprite) = query.single_mut();
    let window = window.single();

    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    let speed = 500.0; // pixels per second
    let movement = direction * speed * time.delta_seconds();

    // Calculate new position
    let new_x = transform.translation.x + movement;

    // Calculate boundaries
    let half_sprite_width = sprite.custom_size.unwrap().x / 2.0;
    let left_bound = -window.width() / 2.0 + half_sprite_width;
    let right_bound = window.width() / 2.0 - half_sprite_width;

    // Clamp the new position within bounds
    transform.translation.x = new_x.clamp(left_bound, right_bound);
}