use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.69, 0.77, 0.87)))
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
