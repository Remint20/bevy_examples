use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::settings::{Backends, WgpuSettings},
};

const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_WIDTH: f32 = 600.0;

fn main() {
    App::new()
        .insert_resource(WgpuSettings {
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
        .insert_resource(MousePos(Vec2::ZERO))
        .add_startup_system(setup)
        .add_system(cursor_system)
        .run();
}

#[derive(Component)]
struct MainCamera;

struct MousePos(Vec2);

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    commands.spawn().insert_bundle(UiCameraBundle::default());
}

fn cursor_system(
    wnds: Res<Windows>,
    mut mouse_pos: ResMut<MousePos>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        if let Some(wnd) = wnds.get_primary() {
            // カメラがウィンドウ内にあるか確認して、その位置を取得する

            if let Some(screen_pos) = wnd.cursor_position() {
                // ウィンドウサイズの取得
                let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

                // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
                let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

                // 投影とカメラ変換を元に戻すための行列
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix.inverse();

                // ワールド座標に変換
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

                // Vec3 -> Vec2に変換
                let world_pos: Vec2 = world_pos.truncate();

                mouse_pos.0 = world_pos;
                eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
            }
        }
    }
}
