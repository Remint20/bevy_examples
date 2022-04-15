use bevy::{
    core::FixedTimestep,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::options::{Backends, WgpuOptions},
};

const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_WIDTH: f32 = 600.0;

const TIME_STEP: f32 = 1.0 / 60.0;

const PLAYER_SPEED: f32 = 120.0;
const MISSILE_SPEED: f32 = 180.0;

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            backends: Some(Backends::VULKAN),
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            title: "SimpleTracking".to_owned(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            // resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement)
                .with_system(tracking_missile_movement),
        )
        .run();
}

#[derive(Component)]
struct TrackingMissile {
    rotation_speed: f32,
}
impl Default for TrackingMissile {
    fn default() -> Self {
        Self {
            rotation_speed: 90_f32.to_radians(),
        }
    }
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-200.0, 0.0, 0.0),
                scale: Vec3::new(0.0625, 0.0625, 1.0),
                ..Default::default()
            },
            texture: assert_server.load("textures/うんちハニワ.png"),
            ..Default::default()
        })
        .insert(TrackingMissile::default());

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(250.0, -250.0, 0.0),
                scale: Vec3::new(0.5, 0.5, 1.0),
                ..Default::default()
            },
            texture: assert_server.load("textures/Ship_C.png"),
            ..Default::default()
        })
        .insert(Player);
}

fn player_movement(
    input_keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let x_dir = if input_keyboard.pressed(KeyCode::A) {
        -1.
    } else if input_keyboard.pressed(KeyCode::D) {
        1.
    } else {
        0.
    };

    let y_dir = if input_keyboard.pressed(KeyCode::W) {
        1.
    } else if input_keyboard.pressed(KeyCode::S) {
        -1.
    } else {
        0.
    };

    if let Ok(mut tf) = query.get_single_mut() {
        tf.translation.x += x_dir * PLAYER_SPEED * TIME_STEP;
        tf.translation.y += y_dir * PLAYER_SPEED * TIME_STEP;
    }
}

fn tracking_missile_movement(
    mut query: Query<(&mut Transform, &mut TrackingMissile), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.truncate();

    for (mut tf, tracking_missile) in query.iter_mut() {
        let missile_translation = tf.translation.truncate();
        // 進んでいる座標方向
        let missile_forward = (tf.rotation * Vec3::Y).truncate();

        // ミサイルからプレイヤーへのベクトル
        let to_player = (player_translation - missile_translation).normalize();

        // 敵の前方ベクトルとプレーヤーへの方向の間の内積を取得
        let forward_dot_player = missile_forward.dot(to_player);

        // 内積が約1.0の場合、敵はすでにプレーヤーに直面しているため、早期に終了できます。
        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

        // ミサイルの正しいベクトルを2Dで取得（すでに単位長）
        let missile_right = (tf.rotation * Vec3::X).truncate();

        // ミサイルの右ベクトルの内積とプレイヤーの船への方向を取得します。
        // 内積が負の場合は反時計回りに回転する必要があり、正の場合は時計回りに回転する必要があります。
        // ドット積が0.0の場合でも、 `copysign`は1.0を返すことに注意してください
        // （プレーヤーが敵の真後ろにいるため、右のベクトルに垂直であるため）。
        let right_dot_player = missile_right.dot(to_player);

        // 右のドットプレーヤーから回転のサインを決定します。
        // 2D bevy座標系は、画面の外を指している+ Zを中心に回転するため、ここで符号を無効にする必要があります。
        // 右手の法則により、+ Zを中心とした正の回転は反時計回りで、負の回転は時計回りです。
        let rotation_sign = -f32::copysign(1.0, right_dot_player);

        // ターゲットをオーバーシュートしないように回転を制限します。
        // ここで内積を角度に変換して、回転角を固定できるようにする必要があります。
        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos();

        // 制限付きの回転角を計算する
        let rotation_angle =
            rotation_sign * (tracking_missile.rotation_speed * TIME_STEP).min(max_angle);

        // クォータニオンを現在の敵の向きの方向からプレーヤーの向きの方向に回転させます
        let rotation_delta = Quat::from_rotation_z(rotation_angle);

        // ミサイルを回転させてプレイヤーと向き合う
        tf.rotation *= rotation_delta;

        let movement_direction = tf.rotation * Vec3::Y;
        let movement_distance = MISSILE_SPEED * TIME_STEP;
        let translation_delta = movement_direction * movement_distance;

        tf.translation += translation_delta;
    }
}
