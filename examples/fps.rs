use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::options::{Backends, WgpuOptions};

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            backends: Some(Backends::VULKAN),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FpsPlugin)
        .run();
}

///
/// FPSを表示するプラグイン
///
/// Usage
///
///
/// ```
/// mod fps;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugin(fps::FpsPlugin)
///     .run();
/// ```
///
pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(fps_setup)
            .add_system(counter_system);
    }
}

#[derive(Component)]
struct StatsText;

fn fps_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "Average FPS",
                TextStyle {
                    font: asset_server.load("fonts/NotoSansJP-Medium.otf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(StatsText);
}

fn counter_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<StatsText>>) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            text.sections[0].value = format!("{:.2}", average);
        }
    }
}
