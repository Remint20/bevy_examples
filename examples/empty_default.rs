// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

///
/// なにも表示されない画面を生成する
///
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
