// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::render::options::{Backends, WgpuOptions};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum GameState {
    Splash,
    Menu,
    // Game,
}

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            backends: Some(Backends::VULKAN),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_state(GameState::Splash)
        .add_plugin(benchmark::BenchMarkPlugin)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(menu::MenuPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

///
/// Component内のオブジェクト全て削除する関数
///
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

///
/// スプラッシュ画面
///
mod splash {
    use bevy::prelude::*;

    use super::{despawn_screen, GameState};

    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            app.add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
                .add_system_set(SystemSet::on_update(GameState::Splash).with_system(countdown))
                .add_system_set(
                    SystemSet::on_exit(GameState::Splash)
                        .with_system(despawn_screen::<OnSplashScreen>),
                );
        }
    }

    #[derive(Component)]
    struct OnSplashScreen;

    struct SplashTimer(Timer);

    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon: Handle<Image> = asset_server.load("textures/うんちハニワ.png");

        commands
            .spawn_bundle(ImageBundle {
                style: Style {
                    margin: Rect::all(Val::Auto),
                    size: Size::new(Val::Px(256.0), Val::Auto),
                    ..Default::default()
                },
                image: UiImage(icon),
                color: UiColor(Color::rgba(1.0, 1.0, 1.0, 1.0)),
                ..Default::default()
            })
            .insert(OnSplashScreen);

        // Splashを表示する時間を指定
        commands.insert_resource(SplashTimer(Timer::from_seconds(1.2, false)));
    }

    fn countdown(
        mut game_state: ResMut<State<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.0.tick(time.delta()).finished() {
            game_state.set(GameState::Menu).unwrap();
        }
    }
}

///
/// メニュー画面
///
mod menu {
    use bevy::prelude::*;

    use super::GameState;

    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
                .add_system_set(
                    SystemSet::on_update(GameState::Menu)
                        .with_system(bevy::input::system::exit_on_esc_system)
                        .with_system(button_system),
                );
        }
    }

    #[derive(Component)]
    struct MenuScreen;

    #[derive(Component)]
    struct SelectedOption;

    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
    const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Quit,
    }

    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut UiColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (i, (interaction, mut color, selected)) in interaction_query.iter_mut().enumerate() {
            *color = match (*interaction, selected) {
                (Interaction::Clicked, _) => {
                    println!("(Clicked, _) {}", i);
                    PRESSED_BUTTON.into()
                }
                (Interaction::Hovered, Some(_)) => {
                    println!("(Hovered, Some(_)) {}", i);
                    HOVERED_PRESSED_BUTTON.into()
                }
                (Interaction::Hovered, None) => {
                    println!("(Hovered, None) {}", i);
                    HOVERED_BUTTON.into()
                }
                (Interaction::None, Some(_)) => {
                    println!("(None, Some(_)) {}", i);
                    PRESSED_BUTTON.into()
                }
                (Interaction::None, None) => {
                    println!("(None, None) {}", i);
                    NORMAL_BUTTON.into()
                }
            }
        }
    }

    fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let font = asset_server.load("fonts/NotoSansJP-Medium.otf");

        let button_style = Style {
            size: Size::new(Val::Px(250.0), Val::Px(65.0)),
            margin: Rect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        };

        let button_text_style = TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: TEXT_COLOR,
        };

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    margin: Rect::all(Val::Auto),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::CRIMSON.into(),
                ..Default::default()
            })
            .insert(MenuScreen)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(50.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "メニュ画面",
                        TextStyle {
                            font: font.clone(),
                            font_size: 80.0,
                            color: TEXT_COLOR.into(),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });

                parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style.clone(),
                        color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    })
                    .insert(MenuButtonAction::Play)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "New Game",
                                button_text_style.clone(),
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    });

                parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style,
                        color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    })
                    .insert(MenuButtonAction::Quit)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section("Quit", button_text_style, Default::default()),
                            ..Default::default()
                        });
                    });
            });
    }
}

///
/// ### BenchMark
///
/// Usage
///
/// ```
/// App()
///     .add_plugins(DefaultPlugins)
///     .add_plugin(benchmark::BenchMarkPlugin)
///     .run();
///
///
mod benchmark {
    use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
    use bevy::prelude::*;

    pub struct BenchMarkPlugin;

    impl Plugin for BenchMarkPlugin {
        fn build(&self, app: &mut App) {
            app.add_startup_system(bench_setup)
                .add_system(counter_system);
        }
    }

    #[derive(Component)]
    struct StatsText;

    fn bench_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}
