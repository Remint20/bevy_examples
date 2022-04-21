// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//
// 当たり判定
//
// 現在: Failed to acquire next swap chain texture!のバグが発生している。
//

use std::path::Path;

use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::render::texture::{CompressedImageFormats, ImageType};
use bevy::sprite::collide_aabb::collide;

const SPRITE_DIR: &str = "assets/textures";
const PLAYER_SPRITE: &str = "ship_a.png";
const ENEMY_SPRITE: &str = "ship_a.png";

struct SpriteInfos {
    player: (Handle<Image>, Vec2),
    enemy: (Handle<Image>, Vec2),
}

fn main() {
    App::new()
        .insert_resource(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system_set_to_stage("", SystemSet::new().with_system(player_spawn))
        // .add_startup_stage(
        //     "spawn",
        //     SystemStage::single(player_spawn).with_system(enemy_spawn),
        // )
        .add_system(collision)
        .add_system(player_movement)
        .run();
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(SpriteInfos {
        player: load_image(&mut images, PLAYER_SPRITE),
        enemy: load_image(&mut images, ENEMY_SPRITE),
    });
}

fn load_image(images: &mut ResMut<Assets<Image>>, path: &str) -> (Handle<Image>, Vec2) {
    let path = Path::new(SPRITE_DIR).join(path);
    let bytes = std::fs::read(&path).expect(&format!("Cannot find {}", path.display()));
    let image = Image::from_buffer(
        &bytes,
        ImageType::MimeType("image/png"),
        CompressedImageFormats::NONE,
        false,
    )
    .unwrap();
    let size = image.size();

    let image_handle = images.add(image);
    (image_handle, size)
}

fn enemy_spawn(mut commands: Commands, sprite_infos: Res<SpriteInfos>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: sprite_infos.enemy.0.clone(),
            transform: Transform {
                translation: Vec3::ZERO,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy);
}

fn player_spawn(mut commands: Commands, sprite_infos: Res<SpriteInfos>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: sprite_infos.player.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., -200., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}

fn collision(
    sprite_infos: Res<SpriteInfos>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok(player_tf) = player_query.get_single() {
        for enemy_tf in enemy_query.iter() {
            let collision = collide(
                player_tf.translation,
                sprite_infos.player.1,
                enemy_tf.translation,
                sprite_infos.enemy.1,
            );

            if let Some(_) = collision {
                println!("collisioned");
            }
        }
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut tf) = query.get_single_mut() {
        let x_direction = if keyboard_input.pressed(KeyCode::A) {
            -1.
        } else if keyboard_input.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };

        let y_direction = if keyboard_input.pressed(KeyCode::S) {
            -1.
        } else if keyboard_input.pressed(KeyCode::W) {
            1.
        } else {
            0.
        };

        tf.translation.x += x_direction;
        tf.translation.y += y_direction;
    }
}
