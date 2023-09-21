use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Small Roguelike".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::DARK_GREEN),
        },
        ..default()
    });

    let texture = asset_server.load("oryx_16bit_scifi_creatures_161.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Player { speed: 100.0 },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        println!("{}", player.speed);

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount * time.delta_seconds();
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}
