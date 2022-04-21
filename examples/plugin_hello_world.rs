// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//
// 画面上に「Hello World」の文字を表示
//

use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.69, 0.77, 0.87)))
        .insert_resource(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::ORANGE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Hello World",
                    TextStyle {
                        font: asset_server.load("fonts/NotoSansJP-Medium.otf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                    TextAlignment::default(),
                ),
                ..Default::default()
            });
        });
}
